use crate::app::AppRoute;
use crate::store::reducer_account::AppDispatch;
use yew::prelude::*;
use yew::services::ConsoleService;
use yew_router::components::RouterAnchor;

pub struct Sidebar {
    dispatch: AppDispatch,
}

pub enum Msg {}

impl Component for Sidebar {
    type Message = Msg;
    type Properties = AppDispatch;

    fn create(dispatch: Self::Properties, _: ComponentLink<Self>) -> Self {
        Sidebar { dispatch }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        type Anchor = RouterAnchor<AppRoute>;
        let account = self.dispatch.state().clone();
        ConsoleService::info(&format!(
            "sidebar account tenant id is {:?}",
            account.tenant_id
        ));
        let tenant_id = if let Some(id) = account.tenant_id {
            id
        } else {
            String::from("no_tenant_id")
        };
        html! {
          <div
            class="jss70 px-sm-3 px-0 bg-white fw-bold h-100"
        >
              <div
              class="d-flex flex-column align-items-center align-items-sm-start py-4 px-3"
              >
                    <div class="MuiDrawer-root MuiDrawer-docked jss68">
                      <nav
                          class="MuiPaper-root MuiDrawer-paper MuiDrawer-paperAnchorLeft jss70 MuiDrawer-paperAnchorDockedLeft MuiPaper-elevation0"
                          style="overflow-y: scroll;"
                          aria-label="primary navigation"
                        >
                          <ul class="jss73" id="nav_accordion">
                              <li class="jss74 nav-item">
                                  <Anchor route=AppRoute::GettingStarted >
                                      <a title="Getting Started" class="jss76" href="#">
                                          <span
                                              class="jss80 jss81">
                                              <i class="fas fa-bolt"></i>
                                          </span>
                                          <span class="jss79">{"Getting Started"}</span>
                                      </a>
                                  </Anchor>
                              </li>
                              <li class="jss74 nav-item">
                                  <Anchor route=AppRoute::Activity>
                                      <a title="Activity" class="jss76" href="#">
                                          <span
                                              class="jss80 jss81">
                                              <i class="fas fa-chart-line"></i>
                                          </span>
                                          <span class="jss79">{"Activity"}</span><span class="jss2 jss103 jss85 jss86 jss82"
                                              data-cosmos-key="label">{"First"}</span>
                                      </a>
                                  </Anchor>
                              </li>
                              <li class="jss74 jss104 nav-item">
                                  <button data-bs-toggle="collapse" data-bs-target="#menu_item1" title="Applications" class="jss76"
                                      aria-expanded="false"
                                      aria-controls="quantum-product-91331-accordion" id="quantum-product-91331">
                                      <span class="jss80 jss81">
                                          <i class="fas fa-server"></i>
                                      </span>
                                      <span id="quantum-product-91331-title" class="jss79">{"Applications"}</span>
                                      <span
                                          class="jss80 jss84">
                                          <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"
                                              fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"
                                              class="jss106">
                                              <polyline points="9 18 15 12 9 6"></polyline>
                                          </svg>
                                      </span>
                                  </button>
                                  <div class="MuiCollapse-container MuiCollapse-hidden" style="min-height: 0px;">
                                      <div class="MuiCollapse-wrapper">
                                          <div class="MuiCollapse-wrapperInner">
                                              <div role="region" id="quantum-product-91331-accordion" aria-labelledby="quantum-product-91331-title">
                                                  <ul id="menu_item1" class="submenu collapse jss105" data-bs-parent="#nav_accordion">
                                                      <li class="jss74">
                                                          <Anchor route=AppRoute::ApplicationHome
                                                          { tenant_id: tenant_id.clone() } >
                                                              <a title="Applications" class="jss76 jss77" href="#"
                                                                  ><span class="jss80 jss81"></span><span
                                                                  class="jss79">{"Applications"}</span></a>
                                                          </Anchor>
                                                      </li>
                                                      <li class="jss74">
                                                          <Anchor route=AppRoute::ApisHome >
                                                              <a title="APIs" class="jss76 jss77" href="#"
                                                                  ><span class="jss80 jss81"></span><span
                                                                  class="jss79">{"APIs"}</span>
                                                              </a>
                                                          </Anchor>
                                                      </li>
                                                      <li class="jss74">
                                                          <Anchor route=AppRoute::SsoHome >
                                                              <a title="SSO Integrations" class="jss76 jss77"
                                                                  href="#"><span class="jss80 jss81"></span><span
                                                                  class="jss79">{"SSO Integrations"}</span></a>
                                                          </Anchor>
                                                      </li>
                                                  </ul>
                                              </div>
                                          </div>
                                      </div>
                                  </div>
                              </li>
                              <li class="jss74 jss104 nav-item">
                                  <button data-bs-toggle="collapse" data-bs-target="#menu_item2"
                                      title="Authentication" class="jss76" aria-expanded="false"
                                      aria-controls="quantum-product-85887-accordion" id="quantum-product-85887">
                                      <span class="jss80 jss81">
                                          <i class="fas fa-fingerprint"></i>
                                      </span>
                                      <span id="quantum-product-85887-title" class="jss79">{"Authentication"}</span>
                                      <span
                                          class="jss80 jss84">
                                          <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"
                                              fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"
                                              class="jss106">
                                              <polyline points="9 18 15 12 9 6"></polyline>
                                          </svg>
                                      </span>
                                  </button>
                                  <div class="MuiCollapse-container MuiCollapse-hidden" style="min-height: 0px;">
                                      <div class="MuiCollapse-wrapper">
                                          <div class="MuiCollapse-wrapperInner">
                                              <div role="region" id="quantum-product-85887-accordion" aria-labelledby="quantum-product-85887-title">
                                                  <ul id="menu_item2" class="jss105 submenu collapse" data-bs-parent="#nav_accordion">
                                                      <li class="jss74">
                                                          <Anchor route=AppRoute::DatabaseHome >
                                                              <a title="Database" class="jss76 jss77" href="#"
                                                                  ><span class="jss80 jss81"></span><span
                                                                  class="jss79">{"Database"}</span></a>
                                                          </Anchor>
                                                      </li>
                                                      <li class="jss74">
                                                          <Anchor route=AppRoute::SocialHome>
                                                              <a title="Social" class="jss76 jss77" href="#"
                                                                  ><span class="jss80 jss81"></span><span
                                                                  class="jss79">{"Social"}</span></a>
                                                          </Anchor>
                                                      </li>
                                                      <li class="jss74">
                                                          <Anchor route=AppRoute::EnterpriseHome>
                                                              <a title="Enterprise" class="jss76 jss77" href="#"
                                                                  ><span
                                                                  class="jss80 jss81"></span><span class="jss79">{"Enterprise"}</span></a>
                                                          </Anchor>
                                                      </li>
                                                      <li class="jss74">
                                                          <Anchor route=AppRoute::AuthPasswordless>
                                                              <a title="Passwordless" class="jss76 jss77" href="#"
                                                                  ><span
                                                                  class="jss80 jss81"></span><span class="jss79">{"Passwordless"}</span></a>
                                                          </Anchor>
                                                      </li>
                                                      <li class="jss74"><a title="Authentication Profile" class="jss76 jss77"
                                                          href="/dashboard/us/dev-test-telkom/authentication-profiles"><span
                                                          class="jss80 jss81"></span><span class="jss79">{"Authentication Profile"}</span></a></li>
                                                  </ul>
                                              </div>
                                          </div>
                                      </div>
                                  </div>
                              </li>
                              <li class="jss74 nav-item">
                                  <a title="Organizations" class="jss76 jss78" aria-current="page"
                                      href="/dashboard/us/dev-test-telkom/organizations">
                                      <span class="jss80 jss81">
                                          <i class="fas fa-building"></i>
                                      </span>
                                      <span class="jss79">{"Organizations"}</span><span class="jss2 jss108 jss85 jss97 jss98 jss82"
                                          data-cosmos-key="label">{"New"}</span>
                                  </a>
                              </li>
                              <li class="jss74 jss104 nav-item">
                                  <button data-bs-toggle="collapse" data-bs-target="#menu_item3"
                                      title="User Management" class="jss76" aria-expanded="false"
                                      aria-controls="quantum-product-60344-accordion" id="quantum-product-60344">
                                      <span class="jss80 jss81">
                                          <i class="fas fa-user-cog"></i>
                                      </span>
                                      <span id="quantum-product-60344-title" class="jss79">{"User Management"}</span>
                                      <span
                                          class="jss80 jss84">
                                          <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"
                                              fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"
                                              class="jss106">
                                              <polyline points="9 18 15 12 9 6"></polyline>
                                          </svg>
                                      </span>
                                  </button>
                                  <div class="MuiCollapse-container MuiCollapse-hidden" style="min-height: 0px;">
                                      <div class="MuiCollapse-wrapper">
                                          <div class="MuiCollapse-wrapperInner">
                                              <div role="region" id="quantum-product-60344-accordion" aria-labelledby="quantum-product-60344-title">
                                                  <ul id="menu_item3" class="submenu collapse jss105" data-bs-parent="#nav_accordion">
                                                      <li class="jss74">
                                                          <Anchor route=AppRoute::UsersManagement {tenant_id: tenant_id.clone()}>
                                                              <a title="Users" class="jss76 jss77" href="#"
                                                                  ><span class="jss80 jss81"></span><span
                                                                  class="jss79">{"Users"}</span></a>
                                                          </Anchor>
                                                      </li>
                                                      <li class="jss74">
                                                          <Anchor route=AppRoute::RolesCreated>
                                                              <a title="Roles" class="jss76 jss77" href="#"
                                                                  ><span class="jss80 jss81"></span><span
                                                                  class="jss79">{"Roles"}</span>
                                                              </a>
                                                          </Anchor>
                                                      </li>
                                                  </ul>
                                              </div>
                                          </div>
                                      </div>
                                  </div>
                              </li>
                              <li class="jss74 jss104 nav-item">
                                  <button data-bs-toggle="collapse" data-bs-target="#menu_item4"
                                      title="Branding" class="jss76" aria-expanded="false"
                                      aria-controls="quantum-product-19902-accordion" id="quantum-product-19902">
                                      <span class="jss80 jss81">
                                          <i class="fab fa-uncharted"></i>
                                      </span>
                                      <span id="quantum-product-19902-title" class="jss79">{"Branding"}</span>
                                      <span
                                          class="jss80 jss84">
                                          <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"
                                              fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"
                                              class="jss106">
                                              <polyline points="9 18 15 12 9 6"></polyline>
                                          </svg>
                                      </span>
                                  </button>
                                  <div class="MuiCollapse-container MuiCollapse-hidden" style="min-height: 0px;">
                                      <div class="MuiCollapse-wrapper">
                                          <div class="MuiCollapse-wrapperInner">
                                              <div role="region" id="quantum-product-19902-accordion" aria-labelledby="quantum-product-19902-title">
                                                  <ul id="menu_item4" class="submenu collapse jss105" data-bs-parent="#nav_accordion">
                                                      <li class="jss74"><a title="Universal Login" class="jss76 jss77"
                                                          href="/dashboard/us/dev-test-telkom/login_settings"><span class="jss80 jss81"></span><span
                                                          class="jss79">{"Universal Login"}</span></a></li>
                                                      <li class="jss74"><a title="Custom Domains" class="jss76 jss77"
                                                          href="/dashboard/us/dev-test-telkom/custom_domains"><span class="jss80 jss81"></span><span
                                                          class="jss79">{"Custom Domains"}</span></a></li>
                                                      <li class="jss74"><a title="Email Templates" class="jss76 jss77"
                                                          href="/dashboard/us/dev-test-telkom/templates"><span class="jss80 jss81"></span><span
                                                          class="jss79">{"Email Templates"}</span></a></li>
                                                      <li class="jss74"><a title="Email Provider" class="jss76 jss77"
                                                          href="/dashboard/us/dev-test-telkom/templates/provider"><span class="jss80 jss81"></span><span
                                                          class="jss79">{"Email Provider"}</span></a></li>
                                                  </ul>
                                              </div>
                                          </div>
                                      </div>
                                  </div>
                              </li>
                              <li class="jss74 jss104 nav-item">
                                  <button data-bs-toggle="collapse" data-bs-target="#menu_item5"
                                      title="Security" class="jss76" aria-expanded="false"
                                      aria-controls="quantum-product-13220-accordion" id="quantum-product-13220">
                                      <span class="jss80 jss81">
                                          <i class="fas fa-shield-alt"></i>
                                      </span>
                                      <span id="quantum-product-13220-title" class="jss79">{"Security"}</span>
                                      <span
                                          class="jss80 jss84">
                                          <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"
                                              fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"
                                              class="jss106">
                                              <polyline points="9 18 15 12 9 6"></polyline>
                                          </svg>
                                      </span>
                                  </button>
                                  <div class="MuiCollapse-container MuiCollapse-hidden" style="min-height: 0px;">
                                      <div class="MuiCollapse-wrapper">
                                          <div class="MuiCollapse-wrapperInner">
                                              <div role="region" id="quantum-product-13220-accordion" aria-labelledby="quantum-product-13220-title">
                                                  <ul id="menu_item5" class="submenu collapse jss105" data-bs-parent="#nav_accordion">
                                                      <li class="jss74"><a title="Attack Protection" class="jss76 jss77"
                                                          href="/dashboard/us/dev-test-telkom/security/attack-protection"><span
                                                          class="jss80 jss81"></span><span class="jss79">{"Attack Protection"}</span></a></li>
                                                      <li class="jss74"><a title="Multi-factor Auth" class="jss76 jss77"
                                                          href="/dashboard/us/dev-test-telkom/security/mfa"><span class="jss80 jss81"></span><span
                                                          class="jss79">{"Multi-factor Auth"}</span></a></li>
                                                      <li class="jss74"><a title="Monitoring" class="jss76 jss77"
                                                          href="/dashboard/us/dev-test-telkom/security/monitoring"><span class="jss80 jss81"></span><span
                                                          class="jss79">{"Monitoring"}</span></a></li>
                                                  </ul>
                                              </div>
                                          </div>
                                      </div>
                                  </div>
                              </li>
                              <li class="jss74 jss104 nav-item">
                                  <button data-bs-toggle="collapse" data-bs-target="#menu_item6" title="Actions"
                                      class="jss76" aria-expanded="false"
                                      aria-controls="quantum-product-85351-accordion" id="quantum-product-85351">
                                      <span class="jss80 jss81">
                                          <i class="fas fa-wave-square"></i>
                                      </span>
                                      <span id="quantum-product-85351-title" class="jss79">{"Actions"}</span>
                                      <span
                                          class="jss80 jss84">
                                          <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"
                                              fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"
                                              class="jss106">
                                              <polyline points="9 18 15 12 9 6"></polyline>
                                          </svg>
                                      </span>
                                  </button>
                                  <div class="MuiCollapse-container MuiCollapse-hidden" style="min-height: 0px;">
                                      <div class="MuiCollapse-wrapper">
                                          <div class="MuiCollapse-wrapperInner">
                                              <div role="region" id="quantum-product-85351-accordion" aria-labelledby="quantum-product-85351-title">
                                                  <ul id="menu_item6" class="submenu collapse jss105" data-bs-parent="#nav_accordion">
                                                      <li class="jss74"><a title="Flows" class="jss76 jss77"
                                                          href="/dashboard/us/dev-test-telkom/actions/flows"><span class="jss80 jss81"></span><span
                                                          class="jss79">{"Flows"}</span></a></li>
                                                      <li class="jss74"><a title="Library" class="jss76 jss77"
                                                          href="/dashboard/us/dev-test-telkom/actions/library"><span class="jss80 jss81"></span><span
                                                          class="jss79">{"Library"}</span></a></li>
                                                  </ul>
                                              </div>
                                          </div>
                                      </div>
                                  </div>
                              </li>
                              <li class="jss74 jss104 nav-item">
                                  <button data-bs-toggle="collapse" data-bs-target="#menu_item7"
                                      title="Auth Pipeline" class="jss76" aria-expanded="false"
                                      aria-controls="quantum-product-18032-accordion" id="quantum-product-18032">
                                      <span class="jss80 jss81">
                                          <i class="fas fa-stream"></i>
                                      </span>
                                      <span id="quantum-product-18032-title" class="jss79">{"Auth Pipeline"}</span>
                                      <span
                                          class="jss80 jss84">
                                          <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"
                                              fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"
                                              class="jss106">
                                              <polyline points="9 18 15 12 9 6"></polyline>
                                          </svg>
                                      </span>
                                  </button>
                                  <div class="MuiCollapse-container MuiCollapse-hidden" style="min-height: 0px;">
                                      <div class="MuiCollapse-wrapper">
                                          <div class="MuiCollapse-wrapperInner">
                                              <div role="region" id="quantum-product-18032-accordion" aria-labelledby="quantum-product-18032-title">
                                                  <ul id="menu_item7" class="submenu collapse jss105" data-bs-parent="#nav_accordion">
                                                      <li class="jss74"><a title="Rules" class="jss76 jss77"
                                                          href="/dashboard/us/dev-test-telkom/rules"><span class="jss80 jss81"></span><span
                                                          class="jss79">{"Rules"}</span></a></li>
                                                      <li class="jss74"><a title="Hooks" class="jss76 jss77"
                                                          href="/dashboard/us/dev-test-telkom/hooks"><span class="jss80 jss81"></span><span
                                                          class="jss79">{"Hooks"}</span></a></li>
                                                  </ul>
                                              </div>
                                          </div>
                                      </div>
                                  </div>
                              </li>
                              <li class="jss74 jss104 nav-item">
                                  <button
                                      data-bs-toggle="collapse" data-bs-target="#menu_item8" title="Monitoring" class="jss76" aria-expanded="false"
                                      aria-controls="quantum-product-9878-accordion" id="quantum-product-9878">
                                      <span class="jss80 jss81">
                                          <i class="fas fa-vector-square"></i>
                                      </span>
                                      <span id="quantum-product-9878-title" class="jss79">{"Monitoring"}</span>
                                      <span
                                          class="jss80 jss84">
                                          <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"
                                              fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"
                                              class="jss106">
                                              <polyline points="9 18 15 12 9 6"></polyline>
                                          </svg>
                                      </span>
                                  </button>
                                  <div class="MuiCollapse-container MuiCollapse-hidden" style="min-height: 0px;">
                                      <div class="MuiCollapse-wrapper">
                                          <div class="MuiCollapse-wrapperInner">
                                              <div role="region" id="quantum-product-9878-accordion" aria-labelledby="quantum-product-9878-title">
                                                  <ul id="menu_item8" class="submenu collapse jss105" data-bs-parent="#nav_accordion">
                                                      <li class="jss74"><a title="Logs" class="jss76 jss77"
                                                          href="/dashboard/us/dev-test-telkom/logs"><span class="jss80 jss81"></span><span
                                                          class="jss79">{"Logs"}</span></a></li>
                                                      <li class="jss74"><a title="Streams" class="jss76 jss77"
                                                          href="/dashboard/us/dev-test-telkom/log-streams"><span class="jss80 jss81"></span><span
                                                          class="jss79">{"Streams"}</span></a></li>
                                                  </ul>
                                              </div>
                                          </div>
                                      </div>
                                  </div>
                              </li>
                              <li class="jss74 nav-item">
                                  <a title="Marketplace" class="jss76"
                                      href="/dashboard/us/dev-test-telkom/marketplace">
                                      <span
                                          class="jss80 jss81">
                                          <i class="fas fa-mail-bulk"></i>
                                      </span>
                                      <span class="jss79">{"Marketplace"}</span>
                                  </a>
                              </li>
                              <li class="jss74 nav-item">
                                  <a title="Extensions" class="jss76"
                                      href="/dashboard/us/dev-test-telkom/extensions/list">
                                      <span
                                          class="jss80 jss81">
                                          <i class="fas fa-border-style"></i>
                                      </span>
                                      <span class="jss79">{"Extensions"}</span>
                                  </a>
                              </li>
                              <li class="jss74 nav-item">
                                  <Anchor
                                      route=AppRoute::SettingsHome
                                  >
                                  <a title="Settings" class="jss76"
                                      href="#">
                                      <span
                                          class="jss80 jss81">
                                          <i class="fas fa-cogs"></i>
                                      </span>
                                      <span class="jss79">{"Settings"}</span>
                                  </a>
                                  </Anchor>
                              </li>
                          </ul>
                      </nav>
                  </div>
              </div>
            </div>
        }
    }
}
