use yew::{
    prelude::*,
    format::{ Json, Nothing },
    services::{
        ConsoleService,
        fetch::{FetchService, FetchTask, Request, Response, StatusCode},
        storage::{ StorageService, Area },
    },
    
};
use yew_router::service::RouteService;
use types::{
    api::{ ApiDetails, ResponseApiDetails },
    LocalStorage,
    LOCALSTORAGE_KEY,
};
use crate::configs::server::API_URL;

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct ApisTabSettingsProps {
    pub api_details: ApiDetails,
}

pub enum StateError {
    Update,
    Delete,
}

pub enum Data {
    // ApiId,
    // Name,
    // Identifier,
    // TokenExp,
    // TokenExpBrowser,
    // SignAlg,
    // Rbac,
    // PermissionAccToken,
    // AllowSkipUser,
    // AllowOffAcc,
    ResourceServerId,
    Name,
    Identifier,
    TokenLifetime,
    TokenLifetimeForWeb,
    SigningAlg,
    EnforcePolicies,
    SkipConsent,
    AllowOfflineAccess,
}

pub struct TabSettings {
    api_details: ApiDetails,
    link: ComponentLink<Self>,
    fetch_task: Option<FetchTask>,
    loading_update_api: bool,
	error_update_api: Option<String>,
    loading_delete_api: bool,
    error_delete_api: Option<String>,
    route_service: RouteService,
    access_token: String,
}

pub enum Msg {
    InputText(String, Data),
    Save,
    GetApiDetails(ApiDetails),
    ResponseError(String, StateError),
    Delete,
    RedirectToApi,
}

impl Component for TabSettings {
    type Message = Msg;
    type Properties = ApisTabSettingsProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        ConsoleService::info(&format!("Api Tab Settings props, api details = {:?}", props.api_details));

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
            api_details: props.api_details,
            link,
            fetch_task: None,
            loading_update_api: false,
		    error_update_api: None,
            loading_delete_api: false,
            error_delete_api: None,
            route_service: RouteService::new(),
            access_token,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::InputText(input, data) => {
              match data {
                Data::ResourceServerId => {
                    self.api_details.resource_server_id = input;
                }
                Data::Name => {
                    self.api_details.name = input;
                }
                Data::Identifier => {
                    self.api_details.identifier = input;
                }
                Data::TokenLifetime => {
                    if input.is_empty() {
                        self.api_details.token_lifetime = 0;
                    } else {
                        self.api_details.token_lifetime = input.parse::<u64>().unwrap();
                    }
                }
                Data::TokenLifetimeForWeb => {
                    if input.is_empty() {
                        self.api_details.token_lifetime_for_web = 0;
                    } else {
                        self.api_details.token_lifetime_for_web = input.parse::<u64>().unwrap();
                    }
                }
                Data::SigningAlg => {
                    self.api_details.signing_alg = input;
                }
                Data::EnforcePolicies => {
                    self.api_details.enforce_policies = !self.api_details.enforce_policies;
                }
                Data::SkipConsent => {
                    self.api_details.skip_consent_for_variable_first_party_clients = !self.api_details.skip_consent_for_variable_first_party_clients;
                }
                Data::AllowOfflineAccess => {
                    self.api_details.allow_offline_access = !self.api_details.allow_offline_access;
                }
              }
              true
            }
            Msg::Save => {
                ConsoleService::info(&format!("{:?}", self.api_details));
                let request = Request::patch(format!("https://evening-cliffs-55855.herokuapp.com/api/v2/resource-server/{}", self.api_details.resource_server_id))
                    .header("Content-Type", "application/json")
                    .header("access_token", self.access_token.clone())
                    .body(Json(&self.api_details))
                    .expect("Could not build request.");
                let callback = self.link.callback(|response: Response<Json<Result<ApiDetails, anyhow::Error>>>| {
                    let Json(data) = response.into_body();
                    match data {
                        Ok(dataok) => {
                            ConsoleService::info(&format!("{:?}", dataok));
                            Msg::GetApiDetails(dataok)
                        }
                        Err(error) => {
                            ConsoleService::info(&error.to_string());
                            Msg::ResponseError(error.to_string(), StateError::Update)
                        }
                    }
                });
                let task = FetchService::fetch(request, callback).expect("failed to start request");
                self.loading_update_api = true;
                self.fetch_task = Some(task);
                true
            }
            Msg::GetApiDetails(data) => {
                self.fetch_task = None;
                self.loading_update_api = false;
                self.api_details = data;
                true
            }
            Msg::ResponseError(message, state) => {
                match state {
                    StateError::Update => {
                        self.fetch_task = None;
                        self.loading_update_api = false;
                        self.error_update_api = Some(message);
                    }
                    StateError::Delete => {
                        self.fetch_task = None;
                        self.loading_delete_api = false;
                        self.error_delete_api = Some(message);
                    }
                }
                true
            }
            Msg::Delete => {
                let request = Request::delete(format!("https://evening-cliffs-55855.herokuapp.com/api/v2/resource-server/{}", self.api_details.resource_server_id))
                    // .header("Content-Type", "application/json")
                    .header("access_token", self.access_token.clone())
                    .body(Nothing)
                    .expect("Could not build request.");
                let callback = self.link.callback(|response: Response<Json<Result<(), anyhow::Error>>>| {
                // let Json(data) = response.into_body();
                let (meta, Json(data)) = response.into_parts();
                
                let status_number = meta.status.as_u16();
                match status_number {
                    204 => {
                        ConsoleService::info("status code is 204");
                        ConsoleService::info("api is deleted");
                        Msg::RedirectToApi
                    }
                    _ => {
                        ConsoleService::info("status code is not 204");
                        match data {
                            Ok(dataok) => {
                                ConsoleService::info(&format!("{:?}", dataok));
                                Msg::RedirectToApi
                            }
                            Err(error) => {
                                ConsoleService::info(&error.to_string());
                                Msg::ResponseError(error.to_string(), StateError::Delete)
                            }
                        }
                    }
                }
                
                });
                let task = FetchService::fetch(request, callback).expect("failed to start request");
                self.loading_delete_api = true;
                self.fetch_task = Some(task);
                true
            }
            Msg::RedirectToApi => {
                self.loading_delete_api = false;
                self.fetch_task = None;
                self.route_service.set_route(&format!("/{}/apis", "tenant_id_not_from_reducer"), ());
                true
            }
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let ApiDetails {
            resource_server_id,
            name,
            is_system: _,
            identifier,
            scopes: _,
            signing_alg,
            signing_secret: _,
            allow_offline_access,
            skip_consent_for_variable_first_party_clients,
            token_lifetime,
            token_lifetime_for_web,
            enforce_policies,
            token_dialect: _,
            client: _,
        } = self.api_details.clone();
        html! {
            <div>

              <div
                  class="container border rounded p-4 d-flex flex-column mb-5"
                  style="font-size: 14px;"
              >
                  <div
                      class="row border-bottom"
                  >
                      <div
                          class="col-lg-6 text-color-primary fw-bold mb-4"
                      >
                          {"General Settings"}
                      </div>
                      <div
                          class="col-lg-6"
                      >
                          <div
                              class="mb-4"
                          >
                              <p class="mb-2 fw-bold">
                                  {"ID"}
                              </p>
                              <div class="input-group mb-2">
                                  <input
                                      type="text"
                                      class="form-control bg-input-grey"
                                      aria-label="Dollar amount (with dot and two decimal places)"
                                      value={resource_server_id}
                                      disabled={true}
                                    //   oninput=self.link.callback(|data: InputData| Msg::InputText(data.value, Data::ResourceServerId))
                                  />   
                              </div>
                              <p>
                                  {"The API id on our system. Useful if you prefer to work directly with Auth0's Management API instead."}
                              </p>
                          </div>
          
                          <div
                              class="mb-4"
                          >
                              <p class="mb-2 fw-bold">
                                  {"Name*"}
                              </p>
                              <div class="input-group mb-2">
                                  <input
                                      type="text"
                                      class="form-control bg-input-grey"
                                      value={name}
                                      oninput=self.link.callback(|data: InputData| Msg::InputText(data.value, Data::Name))
                                  />   
                              </div>
                              <p>
                                  {"A friendly name for the API. The following characters are not allowed "}
                                  <code
                                      style="padding: 2px 6px; font-size: 11px;"
                                      class="bg-input-grey text-color-primary"
                                  >
                                      <i class="bi bi-chevron-left"></i>
                                  </code>
                                  <code
                                      style="padding: 2px 6px; font-size: 11px;"
                                      class="bg-input-grey text-color-primary"
                                  >
                                      <i class="bi bi-chevron-right"></i>
                                  </code>
                              </p>
                          </div>
          
                          <div
                              class="mb-5"
                          >
                              <p class="mb-2 fw-bold">
                                  {"Identifier"}
                              </p>
                              <div class="input-group mb-2">
                                  <input
                                      type="text"
                                      class="form-control bg-input-grey"
                                      aria-label="Dollar amount (with dot and two decimal places)"
                                      value={identifier}
                                      oninput=self.link.callback(|data: InputData| Msg::InputText(data.value, Data::Identifier))
                                  />
                              </div>
                              <p>
                                  {"Unique identifier for the API. This value will be used as the "}
                                  <code
                                      style="padding: 2px 6px; font-size: 14px;"
                                      class="bg-input-grey text-color-primary"
                                  >
                                      {"audience"}
                                  </code>
          
                                  {" parameter on authorization calls."}
                              </p>
                          </div>
          
                      </div>
                  </div>
          
          
                  
                  <div
                      class="row border-bottom mt-5"
                  >
                      <div
                          class="col-lg-6 text-color-primary fw-bold mb-4"
                      >
                          {"Tokens Settings"}
                      </div>
                      <div
                          class="col-lg-6"
                      >
                          <div
                              class="mb-4"
                          >
                              <p class="mb-2 fw-bold">
                                  {"Token Expirations (Seconds)*"}
                              </p>
                              <div class="input-group mb-2">
                                  <input
                                      type="number"
                                      class="form-control bg-input-grey"
                                      aria-label="Dollar amount (with dot and two decimal places)"
                                      value={token_lifetime.to_string()}
                                      oninput=self.link.callback(|data: InputData| Msg::InputText(data.value, Data::TokenLifetime))
                                  />
                              </div>
                              <p>
                                  {"Expiration value (in seconds) for "}
                                  <code
                                      style="padding: 2px 6px; font-size: 11px;"
                                      class="bg-input-grey text-color-primary"
                                  >
                                      {"access tokens"}
                                  </code>
                                  {" issued for this API from the Token Endpoint."}
                              </p>
                          </div>
          
                          <div
                              class="mb-4"
                          >
                              <p class="mb-2 fw-bold">
                                  {"Token Expirations For Browser Flows (Seconds)*"}
                              </p>
                              <div class="input-group mb-2">
                                  <input
                                      type="number"
                                      class="form-control bg-input-grey"
                                      aria-label="Dollar amount (with dot and two decimal places)"
                                      value={token_lifetime_for_web.to_string()}
                                      oninput=self.link.callback(|data: InputData| Msg::InputText(data.value, Data::TokenLifetimeForWeb))
                                  />
                              </div>
                              <p>
                                  {"Expiration value (in seconds) for "}
                                  <code
                                      style="padding: 2px 6px; font-size: 11px;"
                                      class="bg-input-grey text-color-primary"
                                  >
                                      {"access tokens"}
                                  </code>
                                  {" issued for this API via Implicit or Hybrid Flows. Cannot be greater than the Token Lifetime value."}
                              </p>
                          </div>
          
                          <div
                              class="mb-5"
                          >
                              <p class="mb-2 fw-bold">
                                  {"Signing Algorithm"}
                              </p>
                              <div class="input-group mb-2">
                                    <select
                                        class="form-select mb-2"
                                        aria-label="Default select example"
                                        onchange=self.link.callback(|e| {
                                            if let ChangeData::Select(select) = e {
                                                let value = select.value();
                                                Msg::InputText(value, Data::SigningAlg)
                                            } else {
                                                Msg::InputText("No value".to_string(), Data::SigningAlg)
                                            }
                                        })
                                    >
                                        <option
                                            value="RS256"
                                            selected={if signing_alg == "RS256".to_string() {true} else {false}}
                                        >
                                            {"RS256"}
                                        </option>
                                        <option
                                            value="HS256"
                                            selected={if signing_alg == "HS256".to_string() {true} else {false}}
                                        >
                                            {"HS256"}
                                        </option>
                                    </select>
                                //   <input
                                //       type="text"
                                //       class="form-control bg-input-grey"
                                //       aria-label="Dollar amount (with dot and two decimal places)"
                                //       value={signing_alg}
                                //       oninput=self.link.callback(|data: InputData| Msg::InputText(data.value, Data::SigningAlg))
                                //   />
                              </div>
                              <p>
                                  {"Algorithm to be used when signing the "}
                                  <code
                                      style="padding: 2px 6px; font-size: 11px;"
                                      class="bg-input-grey text-color-primary"
                                  >
                                      {"access tokens"}
                                  </code>
          
                                  {" for this API. You can find more information"}
                              </p>
                          </div>
          
                      </div>
                  </div>
          
          
                  <div
                      class="row border-bottom mt-5"
                  >
                      <div
                          class="col-lg-6 text-color-primary fw-bold mb-4"
                      >
                          {"RBAC Settings"}
                      </div>
                      <div
                          class="col-lg-6"
                      >
                          <div
                              class="mb-4"
                          >
                              <p class="mb-2 fw-bold">
                                  {"Enable RBAC "}
                              </p>
                              <div class="form-check form-switch fs-3 mb-4">
                                  <input
                                    class="form-check-input"
                                    type="checkbox"
                                    checked={enforce_policies}
                                    onclick=self.link.callback(|_| Msg::InputText(String::from("none"), Data::EnforcePolicies))
                                />
                              </div>
                              <p class="text-color-disabled">
                                  {"If this setting is enabled, RBAC authorization policies will be enforced for this API. Role and permission assignments will be evaluated during the login transaction."}
                              </p>
                          </div>
                      </div>
                  </div>


                  <div
                      class="row border-bottom mt-5"
                  >
                      <div
                          class="col-lg-6 text-color-primary fw-bold mb-4"
                      >
                          {"Access Settings"}
                      </div>
                      <div
                          class="col-lg-6"
                      >
                          <div
                              class="mb-4"
                          >
                              <p class="mb-2 fw-bold">
                                  {"Allow Skipping User Consent"}
                              </p>
                              <div class="form-check form-switch fs-3 mb-4">
                                  <input
                                    class="form-check-input"
                                    type="checkbox"
                                    checked={skip_consent_for_variable_first_party_clients}
                                    onclick=self.link.callback(|_| Msg::InputText(String::from("none"), Data::SkipConsent))
                                />
                              </div>
                              <p class="text-color-disabled">
                                  {"If this setting is enabled, this API will skip user consent for applications flagged as First Party."}
                              </p>
                          </div>
          
                          <div
                              class="mb-4"
                          >
                              <p class="mb-2 fw-bold">
                                  {"Allow Offline Access"}
                              </p>
                              <div class="form-check form-switch fs-3 mb-4">
                                  <input
                                    class="form-check-input"
                                    type="checkbox"
                                    id="flexSwitchCheckDefault"
                                    checked={allow_offline_access}
                                    onclick=self.link.callback(|_| Msg::InputText(String::from("none"), Data::AllowOfflineAccess))
                                />
                              </div>
                              <p>
                                  {"If this setting is enabled, Auth0 will allow applications to ask for Refresh Tokens for this API."}
                              </p>
                          </div>

                          <div
                            class="mb-5 mt-3"
                          >
                            <button
                              type="button"
                              class=format!("btn {} btn-primary position-relative", if self.loading_update_api {"loading"} else {""} )
                              onclick=self.link.callback(|_| Msg::Save)
                              disabled={ if self.loading_update_api {true} else {false} }
                            >
                              <div class="telkom-label">
                                {"Save"}
                              </div>
                              <div class="telkom-spinner telkom-center">
                                <div class="spinner-border spinner-border-sm" role="status"/>
                              </div>
                            </button>

                            {
                                if self.error_update_api.is_some() {
                                html! {
                                    <div class="alert alert-warning mt-3" role="alert">
                                        <i class="bi bi-exclamation-triangle me-2"></i>
                                        { self.error_update_api.clone().unwrap() }
                                    </div>
                                }
                                } else {
                                    html! {}
                                }
                            }

                          </div>
                      </div>
                  </div>
          
              </div>

              <div
                  style="font-size: 14px;"
              >
                  <p
                      class="fw-bold"
                  >
                      {"Danger Zone"}
                  </p>

                  <div class="alert alert-danger d-flex flex-row justify-content-between" role="alert">
                      <div>
                          <p
                              class="fw-bold"
                          >
                              {"Delete This API"}
                          </p>
                          {"Once confirmed, this operation can't be undone!"}
                      </div>
                      <div>
                        //   <button
                        //       type="button"
                        //       class="btn btn-danger"
                        //   >{"Delete"}</button>

                        <button
                            type="button"
                            class=format!("btn {} btn-danger position-relative", if self.loading_delete_api {"loading"} else {""} )
                            onclick=self.link.callback(|_| Msg::Delete)
                            disabled={ if self.loading_delete_api {true} else {false} }
                        >
                            <div class="telkom-label">
                                {"Delete"}
                            </div>
                            <div class="telkom-spinner telkom-center">
                                <div class="spinner-border spinner-border-sm" role="status"/>
                            </div>
                        </button>
                        {
                            if self.error_delete_api.is_some() {
                                html! {
                                    <div class="alert alert-warning" role="alert">
                                        <i class="bi bi-exclamation-triangle me-2"></i>
                                        { self.error_delete_api.clone().unwrap() }
                                    </div>
                                }
                            } else {
                                html! {}
                            }
                        }
                      </div>
                  </div>
              </div>
          </div>
        }
    }
}
