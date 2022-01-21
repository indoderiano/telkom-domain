use yew::prelude::*;

pub struct UserTabAuthorizedApp {}

pub enum Msg {}

impl Component for UserTabAuthorizedApp {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        UserTabAuthorizedApp {}
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
                <div class="mt-4">
                    <p>{"List of applications that this user has authorized."}</p>
                </div>
                <div class="mt-4">
                        <table class="table table-responsive">
                            <tbody>
                                <tr>
                                    <td>
                                        <a href="">{"My NextJs App"}</a>
                                        <p>{"Audience: https://abangyeska.us.auth0.com/userinfo"}</p>
                                    </td>
                                    <td class="align-middle">
                                        <button type="button" class="btn btn-danger">{"Revoke"}</button>
                                    </td>
                                    <td class="align-middle">
                                        <button type="button" class="btn btn-light" data-bs-toggle="collapse" data-bs-target="#authorAppCollapse">
                                            <i class="bi bi-chevron-down"></i>
                                        </button>
                                    </td>
                                </tr>
                            </tbody>
                        </table>
                    </div>
                    <div class="collapse" id="authorAppCollapse">
                        <div class="card card-body">
                            <p>{"Permissions"}</p>
                            <div>
                                <div class="form-floating text-muted">
                                    <textarea class="form-control" placeholder="Leave a comment here" id="floatingTextarea" disabled=true></textarea>
                                    <label for="floatingTextarea">
                                        <ul class="list-inline">
                                            <li class="list-inline-item">
                                                <div class="form-check">
                                                    <input class="form-check-input" type="checkbox" value="" id="flexCheckChecked" checked=true/>
                                                    <label class="form-check-label" for="flexCheckChecked">{"openid"}</label>
                                                </div>
                                            </li>
                                            <li class="list-inline-item">
                                                <div class="form-check">
                                                    <input class="form-check-input" type="checkbox" value="" id="flexCheckChecked" checked=true/>
                                                    <label class="form-check-label" for="flexCheckChecked">{"profile"}</label>
                                                </div>
                                            </li>
                                         </ul>
                                    </label>
                                </div>        
                            </div>
                        </div>
                    </div>
            </>
        }
    }
}
