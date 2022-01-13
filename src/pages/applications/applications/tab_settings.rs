use yew::{
    prelude::*,
    format::{ Json, Nothing },
    services::{
        ConsoleService,
        fetch::{FetchService, FetchTask, Request, Response, StatusCode},
        storage::{ StorageService, Area },
    },
    agent::Bridged,
    Bridge,
};
use tag_inactive::TagInactive;
use developers_note::DevelopersNote;

use yew_router::{agent::RouteRequest::ChangeRoute, service::RouteService, prelude::*};
use types::{
	application::{ AppDetails, RefreshToken, SigningKeys, JwtConfiguration },
	ResponseMessage,
  LocalStorage,
  LOCALSTORAGE_KEY,
};


#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct AppsTabSettingsProps {
    pub app_details: AppDetails,
}

// #[derive(Clone, Debug, Eq, PartialEq, Properties)]
// pub struct ApisSettingsProps {
//     pub tenant_id: String,
//     // api_title: ApiTitle,
// }

pub enum StateError {
    Update,
    Delete,
}

pub enum Data {
    Tenant,
    Name,
    Domain,
    ClientId,
    ClientSecret,
    AppLogo,
    AppType,
    AuthenticationMethod,
    LoginUrl,
    AllowedUrls ,
    AllowedLogoutUrls ,
    AllowedWebOrigins,
    AllowedOrigins,
    TokenExp,
    RefreshTokenRotation,
    RefreshTokenRotationInterval,
    RefeshTokenAbsoluteExpiration,
    RefeshTokenAbsoluteExpirationLifetime,
    RefeshTokenInactivityExpiration,
    RefeshTokenInactivityExpirationLifetime,
}

pub struct TabSettings {
  app_details: AppDetails,
  link: ComponentLink<Self>,
  fetch_task: Option<FetchTask>,
  loading_update_app: bool,
  error_update_app: Option<String>,
  loading_delete_app: bool,
  error_delete_app: Option<String>,
  route_service: RouteService,
  access_token: String,
  // router_agent: Box<dyn Bridge<RouteAgent>>,
  // tenant_id: String,
}

pub enum Msg {
  InputText(String, Data),
  Save,
  GetAppDetails(AppDetails),
  ResponseError(String, StateError),
  Delete,
  RedirectToApp,
  Ignore
}

impl Component for TabSettings {
    type Message = Msg;
    type Properties = AppsTabSettingsProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {


      let storage = StorageService::new(Area::Local).expect("storage was disabled");
      let localstorage_data = {
          if let Json(Ok(data)) = storage.restore(LOCALSTORAGE_KEY) {
              ConsoleService::info(&format!("{:?}", data));
              data
          } else {
              ConsoleService::info("token does not exist");
              LocalStorage {
                  username: None,
                  email: None,
                  token: None,
              }
          }
      };

      ConsoleService::info(&format!("{:?}", localstorage_data));

      // IF LOCALSTORAGE EXISTS
      // UPDATE STATE
      let mut access_token = String::from("");
      if let Some(_) = localstorage_data.token {
          access_token = localstorage_data.token.unwrap();
      } else {
          
      }

        TabSettings {
          app_details: props.app_details,
          link,
          fetch_task: None,
          loading_update_app: false,
          error_update_app: None,
          loading_delete_app: false,
          error_delete_app: None,
          route_service: RouteService::new(),
          access_token,
          // router_agent: RouteAgent::bridge(link.callback(|_| Msg::Ignore)),
          // tenant_id: "kmzway87aa".to_string()
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
      match msg {
        Msg::InputText(input, data) => {
          match data {
            Data::Tenant => {
                self.app_details.tenant = input;
            }
            Data::Name => {
                self.app_details.name = input;
            }
            Data::Domain => {
                self.app_details.tenant = input;
            }
            Data::ClientId => {
                self.app_details.client_id = input;
            }
            Data::ClientSecret => {
                self.app_details.client_secret = input;
            }
            // Data::AppLogo => {
            //     self.app_details.app_logo = input;
            // }
            Data::AppType => {
                self.app_details.app_type = input;
                ConsoleService::info(&format!("{:?}", self.app_details.app_type.clone()));
            }
            Data::AuthenticationMethod => {
                self.app_details.token_endpoint_auth_method = input;
            }
            // Data::LoginUrl => {
            //     self.app_details.callbacks = input;
            // }
            // Data::AllowedUrls => {
            //     self.app_details.callbacks = input;
            // }
            // Data::AllowedLogoutUrls => {
            //     self.app_details.callbacks = input;
            // }
            // Data::AllowedWebOrigins => {
            //     self.app_details.callbacks = input;
            // }
            // Data::AllowedOrigins => {
            //     self.app_details.allowed_origins = input;
            // }
            Data::TokenExp => {
                if input.is_empty() {
                    self.app_details.refresh_token.token_lifetime = 0;
                } else {
                    self.app_details.refresh_token.token_lifetime = input.parse::<i32>().unwrap();
                }
            }
            // Data::RefreshTokenRotation => {
            //     self.app_details.refresh_token_rotation = !self.app_details.refresh_token_rotation;
            // }
            // Data::RefreshTokenRotationInterval => {
            //     if input.is_empty() {
            //         self.app_details.refresh_token_rotation_interval = 0;
            //     } else {
            //         self.app_details.refresh_token_rotation_interval = input.parse::<i32>().unwrap();
            //     }
            // }
            Data::RefeshTokenAbsoluteExpiration => {
                self.app_details.refresh_token.infinite_token_lifetime = !self.app_details.refresh_token.infinite_token_lifetime;
            }
            Data::RefeshTokenAbsoluteExpirationLifetime => {
                if input.is_empty() {
                    self.app_details.refresh_token.token_lifetime = 0;
                } else {
                    self.app_details.refresh_token.token_lifetime = input.parse::<i32>().unwrap();
                }
            }
            Data::RefeshTokenInactivityExpiration => {
                self.app_details.refresh_token.infinite_idle_token_lifetime = !self.app_details.refresh_token.infinite_idle_token_lifetime;
            }
            Data::RefeshTokenInactivityExpirationLifetime => {
                if input.is_empty() {
                    self.app_details.refresh_token.idle_token_lifetime = 0;
                } else {
                    self.app_details.refresh_token.idle_token_lifetime = input.parse::<i32>().unwrap();
                }
            }
            _ => {
              ()
            }
          }
          true
        }
        Msg::Save => {
          ConsoleService::info(&format!("{:?}", self.app_details));
          let request = Request::patch(format!("https://evening-cliffs-55855.herokuapp.com/api/v2/clients/{}", self.app_details.client_id))
              .header("Content-Type", "application/json")
              .header("access_token", self.access_token.clone())
              .body(Json(&self.app_details))
              .expect("Could not build request.");
          let callback = self.link.callback(|response: Response<Json<Result<AppDetails, anyhow::Error>>>| {
          let Json(data) = response.into_body();
          match data {
              Ok(dataok) => {
                  ConsoleService::info(&format!("{:?}", dataok));
                  Msg::GetAppDetails(dataok)
              }
              Err(error) => {
                  ConsoleService::info(&error.to_string());
                  Msg::ResponseError(error.to_string(), StateError::Update)
              }
          }
          });
          let task = FetchService::fetch(request, callback).expect("failed to start request");
          self.loading_update_app = true;
          self.fetch_task = Some(task);
          true
        }
        Msg::GetAppDetails(data) => {
          self.fetch_task = None;
          self.loading_update_app = false;
          self.app_details = data;
          true
        }
        Msg::ResponseError(message, state) => {
          match state {
            StateError::Update => {
                self.fetch_task = None;
                self.loading_update_app = false;
                self.error_update_app = Some(message);
            }
            StateError::Delete => {
                    self.fetch_task = None;
                    self.loading_delete_app = false;
                    self.error_delete_app = Some(message);
            }
          }
          true
        }
        Msg::Delete => {
          let request = Request::delete(format!("https://evening-cliffs-55855.herokuapp.com/api/v2/clients/{}", self.app_details.client_id))
              .header("Content-Type", "application/json")
              .header("access_token", self.access_token.clone())
              .body(Nothing)
              .expect("Could not build request.");
          let callback = self.link.callback(|response: Response<Json<Result<(), anyhow::Error>>>| {
      
          let (meta, Json(data)) = response.into_parts();
          let status_number = meta.status.as_u16();

          match status_number {
              204 => {
                  ConsoleService::info("status code is 204");
                  ConsoleService::info("api is deleted");
                  Msg::RedirectToApp
              }
              _ => {
                  ConsoleService::info("status code is not 204");
                  match data {
                      Ok(dataok) => {
                          ConsoleService::info(&format!("{:?}", dataok));
                          Msg::RedirectToApp
                      }
                      Err(error) => {
                          ConsoleService::info(&error.to_string());
                          Msg::ResponseError(error.to_string(), StateError::Delete)
                      }
                  }
              }
          }
          // match data {
          //     Ok(dataok) => {
          //         ConsoleService::info(&format!("{:?}", dataok));
          //         // self.router_agent.send(ChangeRoute(<yew_router::route::Route<_> as Trait>::ApplicationHome {self.tenant_id}));
          //         Msg::RedirectToApp
          //     }
          //     Err(error) => {
          //         ConsoleService::info(&error.to_string());
          //         Msg::ResponseError(error.to_string(), StateError::Delete)
          //     }
          // }
          });
          let task = FetchService::fetch(request, callback).expect("failed to start request");
          self.loading_delete_app = true;
          self.fetch_task = Some(task);
          true
        }
        Msg::RedirectToApp => {
          self.loading_delete_app = false;
          self.fetch_task = None;
          self.route_service.set_route(&format!("/{}/apis", self.app_details.tenant), ());
          true
        }
        Msg::Ignore => {true}
      }
    }
        

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
      let AppDetails {
          tenant,
          global,
          is_token_endpoint_ip_header_trusted,
          name,
          is_first_party,
          description,
          oidc_conformant,
          sso_disabled,
          cross_origin_auth,
          allowed_origins,
          web_origins,
          logo_uri,
          sso,
          cross_origin_loc,
          custom_login_page,
          custom_login_page_preview,
          form_template,
          initiate_login_uri,
          organization_usage,
          organization_require_behavior,
          refresh_token,
          encrypted,
          allowed_clients,
          callbacks,
          signing_keys,
          client_id,
          callback_url_template,
          client_secret,
          jwt_configuration,
          client_aliases,
          token_endpoint_auth_method,
          app_type,
          grant_types,
          custom_login_page_on,
          allowed_logout_urls,
        } = self.app_details.clone();

        html! {
            <>
                <div>

      <div class="container border rounded p-4 d-flex flex-column mb-5" style="font-size: 14px;">
        <div class="row border-bottom">
          <div class="col-lg-6 text-color-primary fw-bold mb-4">
            {"Basic Information"}
          </div>
          <div class="col-lg-6">
            <div class="mb-4">
              <p class="mb-2 fw-bold">
                {"Name*"}
              </p>
              <div class="input-group mb-2">
                <input type="text" class="form-control bg-input-grey"
                  aria-label="Dollar amount (with dot and two decimal places)" value={name}
                  oninput=self.link.callback(|data: InputData| Msg::InputText(data.value, Data::Name))
                  />
              </div>
            </div>

            <div class="mb-4">
              <p class="mb-2 fw-bold">
                {"Domain"}
              </p>
              <div class="input-group mb-2">
                <input type="text" class="form-control bg-input-grey"
                  aria-label="Dollar amount (with dot and two decimal places)"
                  value={tenant}
                  oninput=self.link.callback(|data: InputData| Msg::InputText(data.value, Data::Tenant))
                  />
              </div>
            </div>

            <div class="mb-4">
              <p class="mb-2 fw-bold">
                {"Client ID"}
              </p>
              <div class="input-group mb-2">
                <input type="text" class="form-control bg-input-grey"
                  aria-label="Dollar amount (with dot and two decimal places)" value={client_id}
                  oninput=self.link.callback(|data: InputData| Msg::InputText(data.value, Data::ClientId))
                  />
              </div>
            </div>

            <div class="mb-4">
              <p class="mb-2 fw-bold">
                {"Client Secret"}
              </p>
              <div class="input-group mb-2">
                <input type="text" class="form-control bg-input-grey"
                  aria-label="Dollar amount (with dot and two decimal places)" value={client_secret}
                  oninput=self.link.callback(|data: InputData| Msg::InputText(data.value, Data::ClientSecret))
                  />
              </div>
              <p>
                {"The Client Secret is not base64 encoded."}
              </p>
            </div>

            <div class="mb-5">
              <p class="mb-2 fw-bold">
                {"Description"}
              </p>
              <div class="input-group mb-2">
                <textarea class="form-control" rows="4" placeholder="Add a description in less than 140 character"
                value={"Ini description"}
                // oninput=self.link.callback(|data: InputData| Msg::InputText(data.value, Data::Description))
                ></textarea>
              </div>
              <p class="text-color-disabled">
                {"A free text description of the application. Max character count is 140."}
              </p>
            </div>
          </div>
        </div>

        <div class="row border-bottom mt-5">
          <div class="col-lg-6 text-color-primary fw-bold mb-4">
            {"Application Properties"}
          </div>
          <div class="col-lg-6">
            <div class="mb-4">
              <p class="mb-2 fw-bold">
                {"Application Logo"}
              </p>
              <div class="col-md-12 p-2 text-center border rounded p-5"><img height="65px" width="60px"
                  src={"https://cdn.auth0.com/manhattan/versions/1.3226.0/assets/non_interactive.svg"}/></div>
              <div class="input-group mb-2">
                <input type="text" class="form-control bg-input-grey"
                  aria-label="logo-path" value={"https://cdn.auth0.com/manhattan/versions/1.3226.0/assets/non_interactive.svg"}
                  // oninput=self.link.callback(|data: InputData| Msg::InputText(data.value, Data::AppLogo))
                  />
              </div>
              <p>
                {"The URL of the logo to display for the application, if none is set the default badge for this type of
                application will be shown. Recommended size is 150x150 pixels."}
              </p>
            </div>

            <div class="mb-4">
              <p class="mb-2 fw-bold">
                {"Application Type"}
              </p>
              <div class="input-group mb-3">
                <select class="form-select" id="application_type"
                oninput=self.link.callback(|data: InputData| Msg::InputText(data.value, Data::AppType))>
                  <option selected=true>{app_type}</option>
                  <option value="Single Web Application">{"Single Web Application"}</option>
                  <option value="Machine to Machine">{"Machine to Machine"}</option>
                  <option value="Native">{"Native"}</option>
                </select>
              </div>
              <p>
                {"The type of application will determine which settings you can configure from the dashboard."}
              </p>
            </div>

            <div class="mb-4">
              <p class="mb-2 fw-bold">
                {"Token Endpoint Authentication Method"}
              </p>
              <div class="input-group mb-3">
                <select class="form-select" id="application_type" disabled=true>
                  <option selected=true disabled=true>{"None"}</option>
                </select>
              </div>
              <p>
                {"Defines the requested authentication method for the token endpoint. Possible values are 'None' (public
                application without a client secret), 'Post' (application uses HTTP POST parameters) or 'Basic'
                (application uses HTTP Basic)."}
              </p>
            </div>

          </div>
        </div>

        <div class="row border-bottom mt-5">
          <div class="col-lg-6 text-color-primary fw-bold mb-4">
            {"Application URIs"}
          </div>
          <div class="col-lg-6">
            
            <div class="mb-4">
              <p class="mb-2 fw-bold">
                {"Application Login URI"}
              </p>
              <div class="input-group mb-2">
                <input type="text" class="form-control bg-input-grey"
                  aria-label="Dollar amount (with dot and two decimal places)" 
                  value={"for login url"}
                  oninput=self.link.callback(|data: InputData| Msg::InputText(data.value, Data::LoginUrl))
                  />
              </div>
              <p>
                {"In some scenarios, Auth0 will need to redirect to your application’s login page. This URI needs to
                point to a route in your application that should redirect to your tenant’s /authorize endpoint"}
              </p>
            </div>


            <div class="mb-5">
              <p class="mb-2 fw-bold">
                {"Allowed Callback URLs"}
              </p>
              <div class="input-group mb-2">
                <textarea class="form-control" rows="4" value={"allowed url.."}
                // oninput=self.link.callback(|data: InputData| Msg::InputText(data.value, Data::AllowedUrls))
                ></textarea>
              </div>
              <p class="text-color-disabled">
                {"After the user authenticates we will only call back to any of these URLs. You can specify multiple
                valid URLs by comma-separating them (typically to handle different environments like QA or testing).
                Make sure to specify the protocol (https://) otherwise the callback may fail in some cases. With the
                exception of custom URI schemes for native clients, all callbacks should use protocol https://. You can
                use Organization URL parameters in these URLs."}
              </p>
            </div>

            <div class="mb-5">
              <p class="mb-2 fw-bold">
                {"Allowed Logout URLs"}
              </p>
              <div class="input-group mb-2">
                <textarea class="form-control" rows="4"
                  placeholder="Add a description in less than 140 character"
                  value={"allowed logout url"}
                  // oninput=self.link.callback(|data: InputData| Msg::InputText(data.value, Data::AllowedLogoutUrls))
                  ></textarea>
              </div>
              <p class="text-color-disabled">
                {"A set of URLs that are valid to redirect to after logout from Auth0. After a user logs out from Auth0
                you can redirect them with the returnTo query parameter. The URL that you use in returnTo must be listed
                here. You can specify multiple valid URLs by comma-separating them. You can use the star symbol as a
                wildcard for subdomains (*.google.com). Query strings and hash information are not taken into account
                when validating these URLs."}
              </p>
            </div>

            <div class="mb-5">
              <p class="mb-2 fw-bold">
                {"Allowed Web Origins"}
              </p>
              <div class="input-group mb-2">
                <textarea class="form-control" rows="4"
                  placeholder="Add a description in less than 140 character"
                  value={"allowed web origin"}
                  // oninput=self.link.callback(|data: InputData| Msg::InputText(data.value, Data::AllowedWebOrigins))
                  ></textarea>
              </div>
              <p class="text-color-disabled">
                {"Comma-separated list of allowed origins for use with Cross-Origin Authentication, Device Flow, and web
                message response mode, in the form of <scheme> // <host> [ : <port> ], such as
                https://login.mydomain.com or http://localhost:3000. You can use wildcards at the subdomain level
                (e.g.: https://*.contoso.com). Query strings and hash information are not taken into account when
                validating these URLs."}
              </p>
            </div>

            <div class="mb-5">
              <p class="mb-2 fw-bold">
                {"Allowed Origins (CORS)"}
              </p>
              <div class="input-group mb-2">
                <textarea class="form-control" rows="4"
                  placeholder="Add a description in less than 140 character"
                  value={"allowed origin"}
                  // oninput=self.link.callback(|data: InputData| Msg::InputText(data.value, Data::AllowedOrigins))
                  ></textarea>
              </div>
              <p class="text-color-disabled">
                {"Allowed Origins are URLs that will be allowed to make requests from JavaScript to Auth0 API (typically
                used with CORS). By default, all your callback URLs will be allowed. This field allows you to enter
                other origins if you need to. You can specify multiple valid URLs by comma-separating them or one by
                line, and also use wildcards at the subdomain level (e.g.: https://*.contoso.com). Query strings and
                hash information are not taken into account when validating these URLs.. You can use Organization URL
                placeholders in these URLs."}
              </p>
            </div>

          </div>
        </div>

        <div class="row border-bottom mt-5">
          <div class="col-lg-6 text-color-primary fw-bold mb-4">
            {"ID TOKEN"}
          </div>
          <div class="col-lg-6">
            <div class="mb-4">
              <p class="mb-2 fw-bold">
                {"ID Token Expiration "}
              </p>
              <input type="number" class="form-control" min="1" value={refresh_token.token_lifetime.to_string()}width="50px" 
              oninput=self.link.callback(|data: InputData| Msg::InputText(data.value, Data::TokenExp))
              />
              <p class="text-color-disabled">
                {"This setting allows you to set the lifetime of the id_token (in seconds)"}
              </p>
            </div>
          </div>
        </div>

        <div class="row border-bottom mt-5">
          <div class="col-lg-6 text-color-primary fw-bold mb-4">
            {"Refresh Token Rotation"}
          </div>
          <div class="col-lg-6">

            <div class="mb-4">
              <p class="mb-2 fw-bold">
                {"Rotation"}
              </p>
              <div class="form-check form-switch fs-3 mb-4">
                <input class="form-check-input" type="checkbox" 
                // checked={refresh_token_rotation}
                // onclick=self.link.callback(|_| Msg::InputText(String::from("none"), Data::RefreshTokenRotation))
                />
              </div>
              <p class="text-color-disabled">
                {"When enabled, as a result of exchanging a refresh token, a new refresh token will be issued and the
                existing token will be invalidated. This allows for automatic detection of token reuse if the token is
                leaked. In addition, an absolute expiration lifetime must be set."}
              </p>
            </div>

            <div class="mb-4">
              <p class="mb-2 fw-bold">
                {"Reuse Interval"}
              </p>
              <input type="number" class="form-control" min="1" 
              // value={refresh_token_rotation_interval.to_string()}
               width="50px" 
              // oninput=self.link.callback(|data: InputData| Msg::InputText(data.value, Data::RefreshTokenRotationInterval))
              />
              <p class="text-color-disabled">
                {"The allowable leeway time that the same refresh_token can be used to request an access_token without
                triggering automatic reuse detection."}
              </p>
            </div>
          </div>
        </div>

        <div class="row border-bottom mt-5">
          <div class="col-lg-6 text-color-primary fw-bold mb-4">
            {"Refresh Token Expiration"}
          </div>
          <div class="col-lg-6">

            <div class="mb-4">
              <p class="mb-2 fw-bold">
                {"Absolute Expiration"}
              </p>
              <div class="form-check form-switch fs-3 mb-4">
                <input class="form-check-input" type="checkbox" checked={refresh_token.infinite_idle_token_lifetime}
                onclick=self.link.callback(|_| Msg::InputText(String::from("none"), Data::RefeshTokenAbsoluteExpiration))
                />
              </div>
              <p class="text-color-disabled">
                {"When enabled, a refresh_token will expire based on an absolute lifetime, after which the token can no
                longer be used. If rotation is enabled, an expiration lifetime must be set."}
              </p>
            </div>

            <div class="mb-4">
              <p class="mb-2 fw-bold">
                {"Absolute Lifetime"}
              </p>
              <input type="number" class="form-control col-lg-8" min="1" value={refresh_token.token_lifetime.to_string()} width="50px" 
              oninput=self.link.callback(|data: InputData| Msg::InputText(data.value, Data::RefeshTokenAbsoluteExpirationLifetime))
              />
              <p class="text-color-disabled">
                {"Sets the absolute lifetime of a refresh_token (in seconds)."}
              </p>
            </div>

            <div class="mb-4">
              <p class="mb-2 fw-bold">
                {"Inactivity Expiration"}
              </p>
              <div class="form-check form-switch fs-3 mb-4">
                <input class="form-check-input" type="checkbox" checked={refresh_token.infinite_idle_token_lifetime} 
                onclick=self.link.callback(|_| Msg::InputText(String::from("none"), Data::RefeshTokenInactivityExpiration))
                />
              </div>
              <p class="text-color-disabled">
                {"When enabled, a refresh_token will expire based on a specified inactivity lifetime, after which the
                token can no longer be used."}
              </p>
            </div>

            <div class="mb-4">
              <p class="mb-2 fw-bold">
                {"Inactivity Lifetime"}
              </p>
              <input type="number" class="form-control col-lg-8" min="1" value={refresh_token.idle_token_lifetime.to_string()} width="50px" 
              oninput=self.link.callback(|data: InputData| Msg::InputText(data.value, Data::RefeshTokenInactivityExpirationLifetime))
              />
              <p class="text-color-disabled">
                {"Sets the inactivity lifetime of a refresh_token (in seconds)."}
              </p>
            </div>

          </div>
        </div>

        <div class="row border-bottom mt-5">

            <div class="mb-5 mt-3">
              <div class="col-md-12 p-2 text-center">
                <DevelopersNote message="There is still bugfix, work in progress"/>
                <button
                  type="button"
                  class=format!("btn {} btn-primary position-relative", if self.loading_update_app {"loading"} else {""} )
                  onclick=self.link.callback(|_| Msg::Save)
                  disabled={ if self.loading_update_app {true} else {false} }
                >
                  <div class="telkom-label">
                    {"Save Changes"}
                  </div>
                  <div class="telkom-spinner telkom-center">
                    <div class="spinner-border spinner-border-sm" role="status"/>
                  </div>
                </button>
              </div>

              {
                if self.error_update_app.is_some() {
                  html! {
                    <div class="alert alert-warning" role="alert">
                        <i class="bi bi-exclamation-triangle me-2"></i>
                        { self.error_update_app.clone().unwrap() }
                    </div>
                  }
                } else {
                    html! {}
                }
              }

            </div>
          </div>
        </div>

      

      <div style="font-size: 14px;">
        <p class="fw-bold">
          {"Danger Zone"}
        </p>

        <div class="alert alert-danger d-flex flex-row justify-content-between" role="alert">
          <div>
            <p class="fw-bold">
              {"Delete This Application"}
            </p>
            {"Once confirmed, this operation can't be undone!"}
          </div>
          <div>
          <button
            type="button"
            class=format!("btn {} btn-danger position-relative", if self.loading_delete_app {"loading"} else {""} )
            onclick=self.link.callback(|_| Msg::Delete)
            disabled={ if self.loading_delete_app {true} else {false} }
        >
            <div class="telkom-label">
                {"Delete"}
            </div>
            <div class="telkom-spinner telkom-center">
                <div class="spinner-border spinner-border-sm" role="status"/>
            </div>
        </button>
            {
                if self.error_delete_app.is_some() {
                    html! {
                        <div class="alert alert-warning" role="alert">
                            <i class="bi bi-exclamation-triangle me-2"></i>
                            { self.error_delete_app.clone().unwrap() }
                        </div>
                    }
                } else {
                    html! {}
                }
            }
            
          </div>
        </div>
        
        <div class="alert alert-danger d-flex flex-row justify-content-between" role="alert">
          <div>
            <p class="fw-bold">
              {"Rotate secret"}
            </p>
            {"All authorized apps will need to be updated with the new client secret."}
          </div>
          <div>
            <button
              type="button"
              class="btn btn-danger m-auto p-2"
              disabled={true}
            >
              {"Rotate"}
              <TagInactive/>
            </button>

          </div>
        </div>




      </div>
    </div>
                
            </>
    }
  }
}