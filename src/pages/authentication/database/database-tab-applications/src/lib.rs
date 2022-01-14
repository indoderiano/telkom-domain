use yew::prelude::*;
pub struct ApplicationsTab {}
pub enum Msg {}

impl Component for ApplicationsTab {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        ApplicationsTab {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <p class="p-2 m-2">
                {"Applications using this connection."}
                </p>
                <div class="card m-2">
                  <div>
                    <div class="d-flex border-bottom border-1 list-hover">
                      <div class="p-3 d-flex" style="width: 40%;">
                        <div style="flex: 0 0 auto; width: 40px; height: 40px; background-color: #eff0f2;"
                          class="d-flex justify-content-center align-items-center rounded me-3">
                          <img src="https://cdn.auth0.com/manhattan/versions/1.3226.0/assets/non_interactive.svg" style=" color: transparent;
                                                          width: 100%;
                                                          height: 100%;
                                                          object-fit: cover;
                                                          text-align: center;
                                                          text-indent: 10000px;" />
                        </div>
                  
                        <div class="d-grid" style="min-width: 40px;">
                          <a class="fw-bold mb-0" style="
                                                                  white-space: nowrap;
                                                                  text-overflow: ellipsis;
                                                                  overflow: hidden;
                                                                  font-size: 14px;
                                                                  text-decoration: none;
                                                              " href="#">
                            {"Management API (Test Application)"}
                          </a>
                          <p class="mb-0 text-muted" style="
                                                                  white-space: nowrap;
                                                                  text-overflow: ellipsis;
                                                                  overflow: hidden;
                                                                  font-size: 14px;
                                                              ">
                            {"Machine to Machine"}
                          </p>
                        </div>
                  
                      </div>
                  
                      <div class="p-3 d-flex flex-fill align-items-center text-muted">
                      </div>
                  
                      <div class="p-3 d-flex align-items-center dropdown">
                        <div class="form-check form-switch fs-4">
                          <input class="form-check-input" type="checkbox" id="flexSwitchCheckDefault" />
                        </div>
                      </div>
                  
                    </div>
                  </div>
                </div>

                <div class="card m-2">
                  <div>
                    <div class="d-flex border-bottom border-1 list-hover">
                      <div class="p-3 d-flex" style="width: 40%;">
                        <div style="flex: 0 0 auto; width: 40px; height: 40px; background-color: #eff0f2;"
                          class="d-flex justify-content-center align-items-center rounded me-3">
                          <img src="https://cdn.auth0.com/manhattan/versions/1.3226.0/assets/none.svg" style=" color: transparent;
                                                          width: 100%;
                                                          height: 100%;
                                                          object-fit: cover;
                                                          text-align: center;
                                                          text-indent: 10000px;" />
                        </div>
                  
                        <div class="d-grid" style="min-width: 40px;">
                          <a class="fw-bold mb-0" style="
                                                                  white-space: nowrap;
                                                                  text-overflow: ellipsis;
                                                                  overflow: hidden;
                                                                  font-size: 14px;
                                                                  text-decoration: none;
                                                              " href="#">
                            {"Default App"}
                          </a>
                          <p class="mb-0 text-muted" style="
                                                                  white-space: nowrap;
                                                                  text-overflow: ellipsis;
                                                                  overflow: hidden;
                                                                  font-size: 14px;
                                                              ">
                            {"Generic"}
                          </p>
                        </div>
                  
                      </div>
                  
                      <div class="p-3 d-flex flex-fill align-items-center text-muted">
                        
                      </div>
                  
                      <div class="p-3 d-flex align-items-center dropdown">
                        <div class="form-check form-switch fs-4">
                          <input class="form-check-input" type="checkbox" id="flexSwitchCheckDefault" />
                        </div>
                      </div>
                  
                    </div>
                  </div>
                </div>

                <div class="card m-2">
                  <div>
                    <div class="d-flex border-bottom border-1 list-hover">
                      <div class="p-3 d-flex" style="width: 40%;">
                        <div style="flex: 0 0 auto; width: 40px; height: 40px; background-color: #eff0f2;"
                          class="d-flex justify-content-center align-items-center rounded me-3">
                          <img src="https://cdn.auth0.com/manhattan/versions/1.3226.0/assets/spa.svg" style=" color: transparent;
                                                          width: 100%;
                                                          height: 100%;
                                                          object-fit: cover;
                                                          text-align: center;
                                                          text-indent: 10000px;" />
                        </div>
                  
                        <div class="d-grid" style="min-width: 40px;">
                          <a class="fw-bold mb-0" style="
                                                                  white-space: nowrap;
                                                                  text-overflow: ellipsis;
                                                                  overflow: hidden;
                                                                  font-size: 14px;
                                                                  text-decoration: none;
                                                              " href="#">
                            {"My App"}
                          </a>
                          <p class="mb-0 text-muted" style="
                                                                  white-space: nowrap;
                                                                  text-overflow: ellipsis;
                                                                  overflow: hidden;
                                                                  font-size: 14px;
                                                              ">
                            {"Single Page Application"}
                          </p>
                        </div>
                      </div>
                  
                      <div class="p-3 d-flex flex-fill align-items-center text-muted">
                      </div>
                  
                      <div class="p-3 d-flex align-items-center dropdown">
                        <div class="form-check form-switch fs-4">
                          <input class="form-check-input" type="checkbox" id="flexSwitchCheckDefault" />
                        </div>
                      </div>
                    </div>
                  </div>
                </div>

                <div class="card m-2">
                  <div>
                    <div class="d-flex border-bottom border-1 list-hover">
                      <div class="p-3 d-flex" style="width: 40%;">
                        <div style="flex: 0 0 auto; width: 40px; height: 40px; background-color: #eff0f2;"
                          class="d-flex justify-content-center align-items-center rounded me-3">
                          <img src="https://cdn.auth0.com/manhattan/versions/1.3226.0/assets/non_interactive.svg" style=" color: transparent;
                                                          width: 100%;
                                                          height: 100%;
                                                          object-fit: cover;
                                                          text-align: center;
                                                          text-indent: 10000px;" />
                        </div>
                  
                        <div class="d-grid" style="min-width: 40px;">
                          <a class="fw-bold mb-0" style="
                                                                  white-space: nowrap;
                                                                  text-overflow: ellipsis;
                                                                  overflow: hidden;
                                                                  font-size: 14px;
                                                                  text-decoration: none;
                                                              " href="#">
                            {"Test (Test Application)"}
                          </a>
                          <p class="mb-0 text-muted" style="
                                                                  white-space: nowrap;
                                                                  text-overflow: ellipsis;
                                                                  overflow: hidden;
                                                                  font-size: 14px;
                                                              ">
                            {"Machine To Machine"}
                          </p>
                        </div>
                      </div>
                  
                      <div class="p-3 d-flex flex-fill align-items-center text-muted">
                        
                      </div>
                  
                      <div class="p-3 d-flex align-items-center dropdown">
                        <div class="form-check form-switch fs-4">
                          <input class="form-check-input" type="checkbox" id="flexSwitchCheckDefault" />
                        </div>
                      </div>
                    </div>
                  </div>
                </div>
            </>
        }
    }
}