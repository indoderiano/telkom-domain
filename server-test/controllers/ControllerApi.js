
class ControllerApi {
    static get(req, res, next){
        console.log(req.params.tenant_id)
        // res.status(200).json("user")
        // res.status(200).json({
        //     test: 'test'
        // })
        let data = {
            message: "Api fetched",
            data: [
                {
                    id: "60daccd6dff9a6003e8ef6ef",
                    name: "Auth0 Management API",
                    api_type: "System API",
                    identifier: "https://dev-r5y8heyf.au.auth0.com/api/v2/",
                }
            ]
        }

        setTimeout(() => {
            console.log("return data");
            res.send(data)
        }, 3000)
    }

    static create(req, res, next) {
        console.log(req.params.tenant_id)
        console.log(req.body)

    }

    static getDetails(req, res, next) {

        console.log(req.params)

        let data = {
            message: "Api loaded",
            data: {
                id: 1,
                name: "Auth0 Management API",
                api_id: "60daccd6dff9a6003e8ef6ef",
                api_type: "System API",
                identifier: "https://dev-r5y8heyf.au.auth0.com/api/v2/",
                token_exp: 100000,
                token_exp_browser: 10000,
                sign_algorithm: "algorithm signing",
                rbac: true,
                permission_acc_token: true,
                allow_skip_user: true,
                allow_off_acc: true,
                tenant_id: "dev-ofzd5p1b"
            }
        }

        // let data = {
        //     id: 1,
        //     name: "Auth0 Management API",
        //     api_type: "System API",
        //     identifier: "https://dev-r5y8heyf.au.auth0.com/api/v2/",
        //     token_exp: 100000,
        //     token_exp_browser: 10000,
        //     sign_algorithm: "algorithm signing",
        //     rbac: true,
        //     permission_acc_token: true,
        //     allow_skip_user: true,
        //     allow_off_acc: true,
        //     tenant_id: "dev-ofzd5p1b"
        // }

        setTimeout(() => {
            console.log("return data api details");
            res.send(data)
        }, 3000)

    }
    
}

module.exports={
    ControllerApi
}