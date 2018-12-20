pub mod api;
pub mod request;
pub mod types;

#[cfg(test)]
mod tests {
    use reqwest::header::{HeaderMap, CONTENT_TYPE};
    use reqwest::Url;
    use serde::ser::Serialize;
    use serde_json::json;
    use std::collections::HashMap;

    use super::api::Api;
    use super::request::{BasicAuth, HttpReq, MailchimpResult};
    use super::types::*;

    ///
    ///
    /// Pruebas de respuestas para los diferentes ENDPOINTs
    ///
    struct MockRequest {
        resp_for_get: String,
        resp_for_post: String,
    }

    impl MockRequest {
        ///
        /// Inicializador de la clase
        ///
        pub fn new<'m>(resp_for_get: &'m str, resp_for_post: &'m str) -> Self {
            MockRequest {
                resp_for_get: resp_for_get.to_string(),
                resp_for_post: resp_for_post.to_string(),
            }
        }
    }

    impl HttpReq for MockRequest {
        ///
        ///  Argumentos:
        ///     url: Url
        ///     headers: HeaderMap
        ///
        fn get(
            &self,
            _url: Url,
            _headers: HeaderMap,
            _basic_auth: &Option<BasicAuth>,
        ) -> MailchimpResult<String> {
            Ok(self.resp_for_get.clone())
        }
        ///
        ///  Argumentos:
        ///     url: Url
        ///     headers: HeaderMap
        ///     payload: Datos a enviar a la URL especificada
        ///
        fn post<P>(
            &self,
            _url: Url,
            _headers: HeaderMap,
            _payload: P,
            _basic_auth: &Option<BasicAuth>,
        ) -> MailchimpResult<String>
        where
            P: Serialize,
        {
            Ok(self.resp_for_post.clone())
        }
        ///
        ///  Argumentos:
        ///     url: Url
        ///     headers: HeaderMap
        ///     payload: Datos a enviar a la URL especificada
        ///
        fn put<P>(
            &self,
            _url: Url,
            _headers: HeaderMap,
            _payload: P,
            _basic_auth: &Option<BasicAuth>,
        ) -> MailchimpResult<String>
        where
            P: Serialize,
        {
            Ok(self.resp_for_post.clone())
        }
    }

    ///
    /// Configura la instancia de API para los diferentes Test que no requieren de
    /// una respuesta
    ///
    fn setup_test_with_access_token() -> Api<MockRequest> {
        Api::<MockRequest>::new("us6", "access_token", Box::new(MockRequest::new("", "")))
    }

    #[test]
    fn get_domain_url_with_access_token() {
        let api = setup_test_with_access_token();
        assert_eq!(api.domain(), "https://us6.api.mailchimp.com/");
    }
    #[test]
    fn get_api_version_with_access_token() {
        let api = setup_test_with_access_token();
        assert_eq!(api.api_version(), "3.0")
    }

    #[test]
    fn build_url_without_http_params() {
        let api = setup_test_with_access_token();
        assert_eq!(
            api.build_url("lists", &HashMap::new()).as_str(),
            "https://us6.api.mailchimp.com/3.0/lists"
        )
    }
    #[test]
    fn build_url_with_http_params() {
        let api = setup_test_with_access_token();
        let mut params = HashMap::new();
        params.insert("option1".to_string(), "foo".to_string());
        assert_eq!(
            api.build_url("campaigns", &params).as_str(),
            "https://us6.api.mailchimp.com/3.0/campaigns?option1=foo"
        )
    }
    #[test]
    fn build_headers_contain_authorization_header() {
        let api = setup_test_with_access_token();
        let headers = api.build_headers();
        let h_ct_value = headers.get(CONTENT_TYPE).unwrap();
        assert_eq!(h_ct_value.to_str().unwrap(), "application/json");
    }

    #[test]
    fn test_response_for_read_authorized_apps() {
        let mock_transport = MockRequest::new(
            "{ \"apps\": [{\"id\": 2486822,\"name\": \"Mailchimp for Shopify\",\"description\": \"Mailchimp for Shopify is a free application that connects your Shopify store with your Mailchimp account.\",\"users\": [ \"freddiesjokes\"],\"_links\": [ {\"rel\": \"self\",\"href\": \"https://usX.api.mailchimp.com/3.0/authorized-apps/2486822\",\"method\": \"GET\",\"targetSchema\": \"https://api.mailchimp.com/schema/3.0/AuthorizedAppType/Instance.json\" }, {\"rel\": \"parent\",\"href\": \"https://usX.api.mailchimp.com/3.0/authorized-apps\",\"method\": \"GET\",\"targetSchema\": \"https://api.mailchimp.com/schema/3.0/AuthorizedAppType/Collection.json\" }]},{\"id\": 26925,\"name\": \"Integrations Directory Reviews\",\"description\": \"Allow customers to leave feedback on the connect.mailchimp.com site.\",\"users\": [ \"freddiesjokes\"],\"_links\": [ {\"rel\": \"self\",\"href\": \"https://usX.api.mailchimp.com/3.0/authorized-apps/26925\",\"method\": \"GET\",\"targetSchema\": \"https://api.mailchimp.com/schema/3.0/AuthorizedAppType/Instance.json\" }, {\"rel\": \"parent\",\"href\": \"https://usX.api.mailchimp.com/3.0/authorized-apps\",\"method\": \"GET\",\"targetSchema\": \"https://api.mailchimp.com/schema/3.0/AuthorizedAppType/Collection.json\" }]},{\"id\": 27498,\"name\": \"Goooal\",\"description\": \"Segment your Mailchimp list based on subscribers' activity on your website.\",\"users\": [ \"freddiesjokes\"],\"_links\": [ {\"rel\": \"self\",\"href\": \"https://usX.api.mailchimp.com/3.0/authorized-apps/27498\",\"method\": \"GET\",\"targetSchema\": \"https://api.mailchimp.com/schema/3.0/AuthorizedAppType/Instance.json\" }, {\"rel\": \"parent\",\"href\": \"https://usX.api.mailchimp.com/3.0/authorized-apps\",\"method\": \"GET\",\"targetSchema\": \"https://api.mailchimp.com/schema/3.0/AuthorizedAppType/Collection.json\" }]} ], \"total_items\": 3, \"_links\": [{\"rel\": \"self\",\"href\": \"https://usX.api.mailchimp.com/3.0/authorized-apps\",\"method\": \"GET\",\"targetSchema\": \"https://api.mailchimp.com/schema/3.0/AuthorizedAppType/Collection.json\"},{\"rel\": \"parent\",\"href\": \"https://usX.api.mailchimp.com/3.0/\",\"method\": \"GET\",\"targetSchema\": \"https://api.mailchimp.com/schema/3.0/Root.json\"},{\"rel\": \"create\",\"href\": \"https://usX.api.mailchimp.com/3.0/authorized-apps\",\"method\": \"POST\",\"targetSchema\": \"https://api.mailchimp.com/schema/3.0/AuthorizedAppType/ClientAccessTokens.json\",\"schema\": \"https://api.mailchimp.com/schema/3.0/AuthorizedAppType/Client.json\"} ]}",
            "",
        );
        let api = Api::<MockRequest>::new("us6", "access_token", Box::new(mock_transport));

        let expected = json!({ "apps": [{"id": 2486822,"name": "Mailchimp for Shopify","description": "Mailchimp for Shopify is a free application that connects your Shopify store with your Mailchimp account.","users": [ "freddiesjokes"],"_links": [ {"rel": "self","href": "https://usX.api.mailchimp.com/3.0/authorized-apps/2486822","method": "GET","targetSchema": "https://api.mailchimp.com/schema/3.0/AuthorizedAppType/Instance.json" }, {"rel": "parent","href": "https://usX.api.mailchimp.com/3.0/authorized-apps","method": "GET","targetSchema": "https://api.mailchimp.com/schema/3.0/AuthorizedAppType/Collection.json" }]},{"id": 26925,"name": "Integrations Directory Reviews","description": "Allow customers to leave feedback on the connect.mailchimp.com site.","users": [ "freddiesjokes"],"_links": [ {"rel": "self","href": "https://usX.api.mailchimp.com/3.0/authorized-apps/26925","method": "GET","targetSchema": "https://api.mailchimp.com/schema/3.0/AuthorizedAppType/Instance.json" }, {"rel": "parent","href": "https://usX.api.mailchimp.com/3.0/authorized-apps","method": "GET","targetSchema": "https://api.mailchimp.com/schema/3.0/AuthorizedAppType/Collection.json" }]},{"id": 27498,"name": "Goooal","description": "Segment your Mailchimp list based on subscribers' activity on your website.","users": [ "freddiesjokes"],"_links": [ {"rel": "self","href": "https://usX.api.mailchimp.com/3.0/authorized-apps/27498","method": "GET","targetSchema": "https://api.mailchimp.com/schema/3.0/AuthorizedAppType/Instance.json" }, {"rel": "parent","href": "https://usX.api.mailchimp.com/3.0/authorized-apps","method": "GET","targetSchema": "https://api.mailchimp.com/schema/3.0/AuthorizedAppType/Collection.json" }]} ], "total_items": 3, "_links": [{"rel": "self","href": "https://usX.api.mailchimp.com/3.0/authorized-apps","method": "GET","targetSchema": "https://api.mailchimp.com/schema/3.0/AuthorizedAppType/Collection.json"},{"rel": "parent","href": "https://usX.api.mailchimp.com/3.0/","method": "GET","targetSchema": "https://api.mailchimp.com/schema/3.0/Root.json"},{"rel": "create","href": "https://usX.api.mailchimp.com/3.0/authorized-apps","method": "POST","targetSchema": "https://api.mailchimp.com/schema/3.0/AuthorizedAppType/ClientAccessTokens.json","schema": "https://api.mailchimp.com/schema/3.0/AuthorizedAppType/Client.json"} ]});

        let resp = api
            .get_edge::<AuthorizedAppsType>("authorized-apps", HashMap::new())
            .unwrap();

        assert_eq!(
            resp.apps.len(), expected["total_items"],
            "Los estados de la petición no coinciden: Valor de la respuesta {:?} Valor esperado: {:?}",
            resp, expected
        );

        for it in 1..resp.apps.len() {
            assert_eq!(
                resp.apps[it].id, expected["apps"][it]["id"],
                "Los valores no coinciden: Valor respondido {:?} Valor esperado: {:?}",
                resp, expected
            );
        }
    }
    #[test]
    fn test_get_information_about_specific_automation_workflow() {
        let mock_transport = MockRequest::new(
            "{\"id\": \"b0a1c24f1a\",\"create_time\": \"2015-09-15T14:31:54+00:00\",\"start_time\": \"\",\"status\": \"save\",\"emails_sent\": 0,\"recipients\": {\"list_id\": \"1a2df69511\"},\"settings\": {\"title\": \"Freddie's best new jokes\",\"from_name\": \"Freddie\",\"reply_to\": \"freddie@freddiesjokes.com\",\"use_conversation\": false,\"to_name\": \"*|FNAME|*\",\"authenticate\": true,\"auto_footer\": false,\"inline_css\": false},\"tracking\": {\"opens\": true,\"html_clicks\": true,\"text_clicks\": false,\"goal_tracking\": false,\"ecomm360\": true,\"google_analytics\": \"true\",\"clicktale\": \"false\"},\"trigger_settings\": {\"workflow_type\": \"categoryFollowup\",\"send_immediately\": false,\"category_name\": \"Jokes\",\"runtime\": {\"days\": [\"sunday\",\"monday\",\"tuesday\",\"wednesday\",\"thursday\",\"friday\",\"saturday\"],\"hours\": {\"send_asap\": true}},\"workflow_emails_count\": 3},\"_links\": [{\"rel\": \"parent\",\"href\": \"https://usX.api.mailchimp.com/3.0/automations\",\"method\": \"GET\",\"targetSchema\": \"https://api.mailchimp.com/schema/3.0/Automations/Collection.json\",\"schema\": \"https://api.mailchimp.com/schema/3.0/CollectionLinks/Automations.json\"},{\"rel\": \"self\",\"href\": \"https://usX.api.mailchimp.com/3.0/automations/b0a1c24f1a\",\"method\": \"GET\",\"targetSchema\": \"https://api.mailchimp.com/schema/3.0/Automations/Instance.json\"},{\"rel\": \"start-all-emails\",\"href\": \"https://usX.api.mailchimp.com/3.0/automations/b0a1c24f1a/actions/start-all-emails\",\"method\": \"POST\"},{\"rel\": \"pause-all-emails\",\"href\": \"https://usX.api.mailchimp.com/3.0/automations/b0a1c24f1a/actions/pause-all-emails\",\"method\": \"POST\"},{\"rel\": \"emails\",\"href\": \"https://usX.api.mailchimp.com/3.0/automations/b0a1c24f1a/emails\",\"method\": \"GET\",\"targetSchema\": \"https://api.mailchimp.com/schema/3.0/Automations/Emails/Collection.json\"}]}",
            "",
        );
        let api = Api::<MockRequest>::new("us6", "access_token", Box::new(mock_transport));

        let expected = json!({"id": "b0a1c24f1a","create_time": "2015-09-15T14:31:54+00:00","start_time": "","status": "save","emails_sent": 0,"recipients": {"list_id": "1a2df69511"},"settings": {"title": "Freddie's best new jokes","from_name": "Freddie","reply_to": "freddie@freddiesjokes.com","use_conversation": false,"to_name": "*|FNAME|*","authenticate": true,"auto_footer": false,"inline_css": false},"tracking": {"opens": true,"html_clicks": true,"text_clicks": false,"goal_tracking": false,"ecomm360": true,"google_analytics": true,"clicktale": false},"trigger_settings": {"workflow_type": "categoryFollowup","send_immediately": false,"category_name": "Jokes","runtime": {"days": ["sunday","monday","tuesday","wednesday","thursday","friday","saturday"],"hours": {"send_asap": true}},"workflow_emails_count": 3},"_links": [{"rel": "parent","href": "https://usX.api.mailchimp.com/3.0/automations","method": "GET","targetSchema": "https://api.mailchimp.com/schema/3.0/Automations/Collection.json","schema": "https://api.mailchimp.com/schema/3.0/CollectionLinks/Automations.json"},{"rel": "self","href": "https://usX.api.mailchimp.com/3.0/automations/b0a1c24f1a","method": "GET","targetSchema": "https://api.mailchimp.com/schema/3.0/Automations/Instance.json"},{"rel": "start-all-emails","href": "https://usX.api.mailchimp.com/3.0/automations/b0a1c24f1a/actions/start-all-emails","method": "POST"},{"rel": "pause-all-emails","href": "https://usX.api.mailchimp.com/3.0/automations/b0a1c24f1a/actions/pause-all-emails","method": "POST"},{"rel": "emails","href": "https://usX.api.mailchimp.com/3.0/automations/b0a1c24f1a/emails","method": "GET","targetSchema": "https://api.mailchimp.com/schema/3.0/Automations/Emails/Collection.json"}]});

        let resp = api
            .get_edge::<AutomationWorkflowType>("automations/b0a1c24f1a", HashMap::new())
            .unwrap();

        assert_eq!(
            resp.id.as_ref().unwrap().to_string(), expected["id"],
            "Los estados de la petición no coinciden: Valor de la respuesta {:?} Valor esperado: {:?}",
            resp.id, expected["id"]
        );
    }
    #[test]
    fn test_get_list_of_automations() {
        let mock_transport = MockRequest::new(
            "{\"automations\": [{\"id\": \"b0a1c24f1a\",\"create_time\": \"2015-09-15T14:31:54+00:00\",\"start_time\": \"2015-09-15T15:45:32+00:00\",\"status\": \"paused\",\"emails_sent\": 1,\"recipients\": {\"list_id\": \"57afe96172\"},\"settings\": {\"title\": \"Freddie's Best Jokes\",\"from_name\": \"Freddie\",\"reply_to\": \"freddie@freddiesjokes.com\",\"use_conversation\": false,\"to_name\": \"*|FNAME|*\",\"authenticate\": true,\"auto_footer\": false,\"inline_css\": false},\"tracking\": {\"opens\": true,\"html_clicks\": true,\"text_clicks\": true,\"goal_tracking\": true,\"ecomm360\": true,\"google_analytics\": \"Freddie_s_Best_Jokes9_15_2015\",\"clicktale\": \"\"},\"trigger_settings\": {\"workflow_type\": \"emailSeries\",\"send_immediately\": false,\"trigger_on_import\": false,\"runtime\": {\"days\": [\"sunday\",\"monday\",\"tuesday\",\"wednesday\",\"thursday\",\"friday\",\"saturday\"],\"hours\": {\"send_at\": \"12:00am\"}},\"workflow_emails_count\": 1},\"report_summary\": {\"opens\": 1,\"unique_opens\": 1,\"open_rate\": 1,\"clicks\": 0,\"subscriber_clicks\": 0,\"click_rate\": 0},\"_links\": [{\"rel\": \"parent\",\"href\": \"https://usX.api.mailchimp.com/3.0/automations\",\"method\": \"GET\",\"targetSchema\": \"https://api.mailchimp.com/schema/3.0/Automations/Collection.json\",\"schema\": \"https://api.mailchimp.com/schema/3.0/CollectionLinks/Automations.json\"},{\"rel\": \"self\",\"href\": \"https://usX.api.mailchimp.com/3.0/automations/b0a1c24f1a\",\"method\": \"GET\",\"targetSchema\": \"https://api.mailchimp.com/schema/3.0/Automations/Instance.json\"},{\"rel\": \"start-all-emails\",\"href\": \"https://usX.api.mailchimp.com/3.0/automations/b0a1c24f1a/actions/start-all-emails\",\"method\": \"POST\"},{\"rel\": \"pause-all-emails\",\"href\": \"https://usX.api.mailchimp.com/3.0/automations/b0a1c24f1a/actions/pause-all-emails\",\"method\": \"POST\"},{\"rel\": \"emails\",\"href\": \"https://usX.api.mailchimp.com/3.0/automations/b0a1c24f1a/emails\",\"method\": \"GET\",\"targetSchema\": \"https://api.mailchimp.com/schema/3.0/Automations/Emails/Collection.json\"},{\"rel\": \"removed-subscribers\",\"href\": \"https://usX.api.mailchimp.com/3.0/automations/b0a1c24f1a/removed-subscribers\",\"method\": \"GET\",\"targetSchema\": \"https://api.mailchimp.com/schema/3.0/Automations/RemovedSubscribers/Collection.json\"}]}],\"total_items\": 1,\"_links\": [{\"rel\": \"parent\",\"href\": \"https://usX.api.mailchimp.com/3.0/\",\"method\": \"GET\",\"targetSchema\": \"https://api.mailchimp.com/schema/3.0/Root.json\"},{\"rel\": \"self\",\"href\": \"https://usX.api.mailchimp.com/3.0/automations\",\"method\": \"GET\",\"targetSchema\": \"https://api.mailchimp.com/schema/3.0/Automations/Collection.json\",\"schema\": \"https://api.mailchimp.com/schema/3.0/CollectionLinks/Automations.json\"}]}",
            "",
        );
        let api = Api::<MockRequest>::new("us6", "access_token", Box::new(mock_transport));
        let resp = api
            .get_edge::<AutomationsType>("/automations", HashMap::new())
            .unwrap();

        assert_eq!(
            resp.automations.len(), resp.total_items as usize,
            "Los estados de la petición no coinciden: Valor de la respuesta {:?} Valor esperado: {:?}",
            resp.automations.len(), resp.total_items
        );
    }

    #[test]
    fn test_automations_pause_all_emails() {
        let mock_transport = MockRequest::new("", "");
        let api = Api::<MockRequest>::new("us6", "access_token", Box::new(mock_transport));
        let resp = api.post_edge::<EmptyType, HashMap<String, String>>(
            "/automations/fd9d304eb7/actions/pause-all-emails",
            HashMap::new(),
        );

        match resp {
            Ok(_) => assert_eq!(true, true),
            Err(e) => assert_eq!(
                false, true,
                "Los estados de la petición no coinciden: Valor de la respuesta {:?}",
                e
            ),
        }
    }

    #[test]
    fn test_api_root() {
        let mock_transport = MockRequest::new(
            "{\"account_id\": \"8d3a3db4d97663a9074efcc16\",\"account_name\": \"Freddie's Jokes\",\"email\": \"freddie@mailchimp.com\",\"role\": \"owner\",\"contact\": {\"company\": \"Freddie's Jokes\",\"addr1\": \"675 Ponce De Leon Ave NE\",\"addr2\": \"Suite 5000\",\"city\": \"Atlanta\",\"state\": \"GA\",\"zip\": \"30308\",\"country\": \"US\"},\"last_login\": \"2015-09-15 14:25:37\",\"total_subscribers\": 413,\"_links\": [{\"rel\": \"self\",\"href\": \"https://usX.api.mailchimp.com/3.0/\",\"method\": \"GET\",\"targetSchema\": \"https://api.mailchimp.com/schema/3.0/Root.json\"},{\"rel\": \"lists\",\"href\": \"https://usX.api.mailchimp.com/3.0/lists\",\"method\": \"GET\",\"targetSchema\": \"https://api.mailchimp.com/schema/3.0/Lists/Collection.json\",\"schema\": \"https://api.mailchimp.com/schema/3.0/CollectionLinks/Lists.json\"},{\"rel\": \"reports\",\"href\": \"https://usX.api.mailchimp.com/3.0/reports\",\"method\": \"GET\",\"targetSchema\": \"https://api.mailchimp.com/schema/3.0/Reports/Collection.json\",\"schema\": \"https://api.mailchimp.com/schema/3.0/CollectionLinks/Reports.json\"},{\"rel\": \"conversations\",\"href\": \"https://usX.api.mailchimp.com/3.0/conversations\",\"method\": \"GET\",\"targetSchema\": \"https://api.mailchimp.com/schema/3.0/Conversations/Collection.json\",\"schema\": \"https://api.mailchimp.com/schema/3.0/CollectionLinks/Conversations.json\"},{\"rel\": \"campaigns\",\"href\": \"https://usX.api.mailchimp.com/3.0/campaigns\",\"method\": \"GET\",\"targetSchema\": \"https://api.mailchimp.com/schema/3.0/Campaigns/Collection.json\",\"schema\": \"https://api.mailchimp.com/schema/3.0/CollectionLinks/Campaigns.json\"},{\"rel\": \"automations\",\"href\": \"https://usX.api.mailchimp.com/3.0/automations\",\"method\": \"GET\",\"targetSchema\": \"https://api.mailchimp.com/schema/3.0/Automations/Collection.json\",\"schema\": \"https://api.mailchimp.com/schema/3.0/CollectionLinks/Automations.json\"},{\"rel\": \"templates\",\"href\": \"https://usX.api.mailchimp.com/3.0/templates\",\"method\": \"GET\",\"targetSchema\": \"https://api.mailchimp.com/schema/3.0/Templates/Collection.json\",\"schema\": \"https://api.mailchimp.com/schema/3.0/CollectionLinks/Templates.json\"},{\"rel\": \"file-manager\",\"href\": \"https://usX.api.mailchimp.com/3.0/file-manager\",\"method\": \"GET\",\"targetSchema\": \"https://api.mailchimp.com/schema/3.0/FileManager/Namespace.json\"},{\"rel\": \"authorized-apps\",\"href\": \"https://usX.api.mailchimp.com/3.0/authorized-apps\",\"method\": \"GET\",\"targetSchema\": \"https://api.mailchimp.com/schema/3.0/AuthorizedApps/Collection.json\"}]}",
            "",
        );
        let api = Api::<MockRequest>::new("us6", "access_token", Box::new(mock_transport));
        let resp = api.get_edge::<ApiRootType>("", HashMap::new()).unwrap();

        assert_eq!(
            resp.account_id, "8d3a3db4d97663a9074efcc16",
            "Los estados de la petición no coinciden: Valor de la respuesta {:?} Valor esperado: {:?}",
            resp.account_id, "8d3a3db4d97663a9074efcc16"
        );
    }
    #[test]
    fn test_workflow_email() {
        let mock_transport = MockRequest::new(
            "{\"id\": \"491fec26f1\",\"workflow_id\": \"b0a1c24f1a\",\"position\": 1,\"delay\": {\"amount\": 1,\"type\": \"day\",\"direction\": \"after\",\"action\": \"signup\"},\"create_time\": \"2015-09-15T14:33:20+00:00\",\"start_time\": \"2015-09-15T15:45:32+00:00\",\"archive_url\": \"http://eepurl.com/xxxx\",\"status\": \"paused\",\"emails_sent\": 0,\"send_time\": \"2015-09-15T15:48:05+00:00\",\"content_type\": \"template\",\"recipients\": {\"list_id\": \"57afe96172\"},\"settings\": {\"subject_line\": \"Your first joke from Freddie!\",\"title\": \"Freddie Likes Jokes\",\"from_name\": \"Freddie\",\"reply_to\": \"freddie@freddiesjokes.com\",\"authenticate\": false,\"auto_footer\": false,\"inline_css\": false,\"auto_tweet\": false,\"fb_comments\": true,\"template_id\": 2000020,\"drag_and_drop\": true},\"tracking\": {\"opens\": true,\"html_clicks\": true,\"text_clicks\": true,\"goal_tracking\": true,\"ecomm360\": true,\"google_analytics\": \"true\",\"clicktale\": \"\"},\"report_summary\": {\"opens\": 0,\"unique_opens\": 0,\"open_rate\": 0,\"clicks\": 0,\"subscriber_clicks\": 0,\"click_rate\": 0},\"_links\": [{\"rel\": \"parent\",\"href\": \"https://usX.api.mailchimp.com/3.0/automations/b0a1c24f1a/emails\",\"method\": \"GET\",\"targetSchema\": \"https://api.mailchimp.com/schema/3.0/Automations/Emails/Collection.json\"},{\"rel\": \"self\",\"href\": \"https://usX.api.mailchimp.com/3.0/automations/b0a1c24f1a/emails/491fec26f1\",\"method\": \"GET\",\"targetSchema\": \"https://api.mailchimp.com/schema/3.0/Automations/Emails/Instance.json\"},{\"rel\": \"start\",\"href\": \"https://usX.api.mailchimp.com/3.0/automations/b0a1c24f1a/emails/491fec26f1/actions/start\",\"method\": \"POST\"},{\"rel\": \"pause\",\"href\": \"https://usX.api.mailchimp.com/3.0/automations/b0a1c24f1a/emails/491fec26f1/actions/pause\",\"method\": \"POST\"},{\"rel\": \"queue\",\"href\": \"https://usX.api.mailchimp.com/3.0/automations/b0a1c24f1a/emails/491fec26f1/queue\",\"method\": \"GET\",\"targetSchema\": \"https://api.mailchimp.com/schema/3.0/Automations/Emails/Queue/Collection.json\"}]}",
            "",
        );
        let api = Api::<MockRequest>::new("us6", "access_token", Box::new(mock_transport));
        let resp = api.get_edge::<WorkflowEmailType>("", HashMap::new()).unwrap();

        assert_eq!(
            resp.id.as_ref().unwrap(), "491fec26f1",
            "Los estados de la petición no coinciden: Valor de la respuesta {:?} Valor esperado: {:?}",
            resp.id.as_ref().unwrap(), "491fec26f1"
        );
    }

}
