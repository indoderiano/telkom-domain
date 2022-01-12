class ControllerUsers {
    static get(req, res, next) {
        console.log("get user list")
        console.log(req.params.tenant_id);

        let data = {
            message: "Ok",
            data: [{
                    user_id: "auth0|YXV0aDAlN0M2MTM3MTIyMTAxY2VmYTAwNzM0NzRmYmI",
                    email: "yeskahaganta3838@gmail.com",
                    email_verified: false,
                    username: "abangyeska",
                    phone_number: "+6281239675121",
                    phone_verified: false,
                    created_at: "2021-09-07T07:17:53.901Z",
                    updated_at: "2021-09-28T07:16:17.611Z",
                    identities: [{
                        connection: "Initial-Connection",
                        user_id: "YXV0aDAlN0M2MTM3MTIyMTAxY2VmYTAwNzM0NzRmYmI",
                        provider: "auth0",
                        is_social: false,
                    }, ],
                    app_metadata: {},
                    user_metadata: {},
                    picture: "https://s.gravatar.com/avatar/be4a14d4c9b0cd03d487ac71f40d6047?s=480&r=pg&d=https%3A%2F%2Fcdn.auth0.com%2Favatars%2Fye.png",
                    name: "Yeska Haganta",
                    nickname: "brotheryeska",
                    multifactor: [""],
                    last_ip: "36.92.133.42",
                    last_login: "2021-09-28T07:16:17.610Z",
                    logins_count: 1,
                    blocked: false,
                    given_name: "Yeska",
                    family_name: "Sembiring",
                },
                {
                    user_id: "auth0|YXV0aDAlN0M2MTM3MTIyMTAxY2VmYTAwNzM0NzRmYmK",
                    email: "fransyogabarus@gmail.com",
                    email_verified: false,
                    username: "abangyoga",
                    phone_number: "+628188812121212",
                    phone_verified: false,
                    created_at: "2021-09-07T07:17:58.901Z",
                    updated_at: "2021-09-28T07:16:19.611Z",
                    identities: [{
                        connection: "Initial-Connection2",
                        user_id: "YXV0aDAlN0M2MTM3MTIyMTAxY2VmYTAwNzM0NzRmYmK",
                        provider: "auth0",
                        is_social: false,
                    }, ],
                    app_metadata: {},
                    user_metadata: {},
                    picture: "https://s.gravatar.com/avatar/be4a14d4c9b0cd03d487ac71f40d6047?s=480&r=pg&d=https%3A%2F%2Fcdn.auth0.com%2Favatars%2Fye.png",
                    name: "Frans Yoga",
                    nickname: "abangagoy",
                    multifactor: [""],
                    last_ip: "37.92.133.44",
                    last_login: "2021-09-28T07:17:17.610Z",
                    logins_count: 1,
                    blocked: false,
                    given_name: "Frans",
                    family_name: "Barus",
                },
            ],
        };

        setTimeout(() => {
            console.log("return users data");
            res.send(data);
        }, 3000);
    }

    static create(req, res, next) {
        console.log(req.params.tenant_id);
        console.log(req.body);

        setTimeout(() => {
            res.send({
                message: "create succesful",
                data: "",
            });
        }, 3000);
    }

    static getDetails(req, res, next) {
        console.log(req.params, "ini di get details");

        let data = {
            user_id: "auth0|YXV0aDAlN0M2MTM3MTIyMTAxY2VmYTAwNzM0NzRmYmI",
            email: "yeskahaganta3838@gmail.com",
            email_verified: false,
            username: "abangyeska",
            phone_number: "+6281239675121",
            phone_verified: false,
            created_at: "2021-09-07T07:17:53.901Z",
            updated_at: "",
            identities: [{
                connection: "Initial-Connection",
                user_id: "YXV0aDAlN0M2MTM3MTIyMTAxY2VmYTAwNzM0NzRmYmI",
                provider: "auth0",
                is_social: false,
            }, ],
            app_metadata: {},
            user_metadata: {},
            picture: "https://s.gravatar.com/avatar/be4a14d4c9b0cd03d487ac71f40d6047?s=480&r=pg&d=https%3A%2F%2Fcdn.auth0.com%2Favatars%2Fye.png",
            name: "Yeska Haganta",
            nickname: "brotheryeska",
            multifactor: [""],
            last_ip: "36.92.133.42",
            last_login: "2021-09-28T07:16:17.610Z",
            logins_count: 1,
            blocked: false,
            given_name: "Yeska",
            family_name: "Sembiring",
        };

        setTimeout(() => {
            console.log("return data user details");
            res.send(data);
        }, 3000);
    }

    static updateUsers(req, res, next) {
        console.log(req.params)
        console.log("ini di update)")

        let data = {
            user_id: "auth0|YXV0aDAlN0M2MTM3MTIyMTAxY2VmYTAwNzM0NzRmYmI",
            email: "yeskahaganta3838@gmail.com",
            email_verified: false,
            username: "abangyeska",
            phone_number: "+6281239675121",
            phone_verified: false,
            created_at: "2021-09-07T07:17:53.901Z",
            updated_at: "",
            identities: [{
                connection: "Initial-Connection",
                user_id: "YXV0aDAlN0M2MTM3MTIyMTAxY2VmYTAwNzM0NzRmYmI",
                provider: "auth0",
                is_social: false,
            }, ],
            app_metadata: {},
            user_metadata: {},
            picture: "https://s.gravatar.com/avatar/be4a14d4c9b0cd03d487ac71f40d6047?s=480&r=pg&d=https%3A%2F%2Fcdn.auth0.com%2Favatars%2Fye.png",
            name: "Yeska Haganta",
            nickname: "brotheryeska",
            multifactor: [""],
            last_ip: "36.92.133.42",
            last_login: "2021-09-28T07:16:17.610Z",
            logins_count: 1,
            blocked: false,
            given_name: "Yeska",
            family_name: "Sembiring",
        };

        setTimeout(() => {
            console.log("return data user diupdate");
            res.send(data);
        }, 1000);

    }


    static getLogEvent(req, res, next) {
        console.log(req.params.id)
        console.log("get log event")

        let data = [{
                date: "2011-10-30T01:48:00.000Z",
                type_detail: "s",
                description: "Successful Login",
                connection: "google-oauth2",
                connection_id: "con_mZOXCcKwOs7V0MGq",
                client_id: "XMyKpNTUzI1izvSPY8JJ8ZIIO5izaJnH",
                client_name: "My NextJs App",
                ip: "190.257.209.19",
                hostname: "190.257.209.19",
                user_id: "google-oauth2|114519841297346872283",
                user_name: "Yeska Haganta",
                audience: "",
                scope: "",
                strategy: "",
                strategy_type: "",
                log_id: "90020211030011805058892884646997415862276415694263287810",
                is_mobile: false,
                details: {},
                user_agent: "",
                location_info: {
                    country_code: "ID",
                    country_code_3: "IDN",
                    country_name: "Indonesia",
                    city_name: "Tangerang",
                    latitude: "",
                    longitude: "",
                    time_zone: "",
                    continent_code: "AS",
                },
            },
            {
                date: "2011-10-30T01:50:00.000Z",
                type_detail: "slo",
                description: "Successful Logout",
                connection: "google-oauth2",
                connection_id: "con_mZOXCcKwOs7V0MGq",
                client_id: "XMyKpNTUzI1izvSPY8JJ8ZIIO5izaJnH",
                client_name: "My NextJs App",
                ip: "190.257.209.19",
                hostname: "190.257.209.19",
                user_id: "google-oauth2|114519841297346872283",
                user_name: "Yeska Haganta",
                audience: "",
                scope: "",
                strategy: "",
                strategy_type: "",
                log_id: "90020211030011824658252733388425069519077906452486029330",
                is_mobile: false,
                details: {},
                user_agent: "",
                location_info: {
                    country_code: "ID",
                    country_code_3: "IDN",
                    country_name: "Indonesia",
                    city_name: "Tangerang",
                    latitude: "",
                    longitude: "",
                    time_zone: "",
                    continent_code: "AS",
                },
            }
        ];

        setTimeout(() => {
            console.log("return log data details");
            res.send(data);
        }, 3000);

    }

    static getRoles(req, res, next) {
        console.log(req.params.id)
        let data = [
            {
                id: "rol_mIPAEs3z2jA4QnEz",
                name: "admin",
                description: "Admin role for my app"
            },
            {
                id: "rol_mIPAEsasdfaseadd",
                name: "agent",
                description: "Agent role for my app"
            },
            {
                id: "rol_mIPAE4QnEz",
                name: "client",
                description: "Client role for my app"
            },
        ]


        setTimeout(() => {
            console.log("return log role details");
            res.send(data);
        }, 3000)
    }

    static getUserPermissions(req, res, next) {
        console.log(req.params.id)

        let data = [{
                resource_server_identifier: "https://jsonplaceholder.typicode.com/albums",
                permission_name: "read:client_grants",
                resource_server_name: "Example API",
                description: "Read API"
            },
            {
                resource_server_identifier: "https://jsonplaceholder.typicode.com/albums",
                permission_name: "update:client_grants",
                resource_server_name: "Example API",
                description: "Update Data"
            },
            {
                resource_server_identifier: "https://jsonplaceholder.typicode.com/albums",
                permission_name: "delete:client_grants",
                resource_server_name: "Example API",
                description: "Delete API"
            },
        ]

        setTimeout(() => {
            console.log("return log role details");
            res.send(data);
        }, 3000)

    }


    static deleteUser(req, res) {
        console.log(req.params)
        console.log(req.body)
        console.log("User berhasil di delete")

        setTimeout(() => {
            console.log("return user deleted");
            res.send({
                message: "User deleted",
                data: "",
            })
        }, 3000)

    }

    static deleteRolesFromUser(req, res) {
        console.log(req.params)
        console.log(req.body)
        console.log("Roles dari User berhasil di delete")

        setTimeout(() => {
            console.log("return roles deleted");
            res.send({
                message: "Roles User deleted",
                data: "",
            })
        }, 3000)

    }

    static deletePermissionsFromUser(req, res) {
        console.log(req.params)
        console.log(req.body)
        console.log("Permissions dari User berhasil di delete")

        setTimeout(() => {
            console.log("return permissions deleted");
            res.send({
                message: "Permissions User deleted",
                data: "",
            })
        }, 3000)
    }

}

module.exports = {
    ControllerUsers,
};