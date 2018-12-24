use super::ecommerce::ECommerceReportType;
use super::link::LinkType;

// ============ Segment Conditions ==============
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SegmentConditionsType {
    /// The type of segment, for example: date, language, Mandrill, static, and more.
    #[serde(default)]
    pub condition_type: String,
}

impl Default for SegmentConditionsType {
    fn default() -> Self {
        SegmentConditionsType {
            condition_type: "".to_string(),
        }
    }
}

// ============ Segment Options ==============
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SegmentOptionsType {
    /// The id for an existing saved segment.
    #[serde(default)]
    pub saved_segment_id: u64,
    /// The prebuilt segment id, if a prebuilt segment has been designated for this campaign.
    #[serde(default)]
    pub prebuilt_segment_id: String,
    /// Desc: Segment match type.
    #[serde(default, rename = "match")]
    pub match_filter: String,
    /// An array of segment conditions.
    #[serde(default)]
    pub conditions: Vec<SegmentConditionsType>,
}

impl Default for SegmentOptionsType {
    fn default() -> Self {
        SegmentOptionsType {
            saved_segment_id: 0,
            prebuilt_segment_id: "".to_string(),
            match_filter: "".to_string(),
            conditions: Vec::new(),
        }
    }
}

// ============ Automation Delay ==============
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AutomationDelayType {
    /// The delay amount for an automation email.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<u64>,
    /// The type of delay for an automation email.
    #[serde(default, rename = "type", skip_serializing_if = "Option::is_none")]
    pub delay_type: Option<String>,
    /// Whether the delay settings describe before or after the delay action of an automation email.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,
    /// The action that triggers the delay of an automation emails.

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
}

impl Default for AutomationDelayType {
    fn default() -> Self {
        AutomationDelayType {
            amount: Some(0),
            delay_type: Some("".to_string()),
            direction: Some("".to_string()),
            action: Some("".to_string()),
        }
    }
}

// ============ Recipient ==============
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RecipientType {
    /// The unique list id.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub list_id: Option<String>,
    /// Desc: The status of the list used, namely if it’s deleted or disabled.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub list_is_active: Option<bool>,
    /// Desc: List Name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub list_name: Option<String>,
    /// Desc: A description of the segment used for the campaign. Formatted as a string marked up with HTML.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub segment_text: Option<String>,
    /// Desc: Count of the recipients on the associated list. Formatted as an integer..
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recipient_count: Option<u64>,
    /// Desc: An object representing all segmentation options.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub segment_opts: Option<SegmentOptionsType>,
    /// Desc: The id of the store.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub store_id: Option<String>,
}

impl Default for RecipientType {
    fn default() -> Self {
        RecipientType {
            list_id: None,
            list_is_active: None,
            segment_text: None,
            list_name: None,
            recipient_count: None,
            segment_opts: None,
            store_id: None,
        }
    }
}

impl RecipientType {
    ///
    /// Función de ayuda para el proceso creación de una automatización
    ///
    /// Argumentos:
    ///     list_id: Id de la lista
    ///     store_id: Id de la store
    ///
    pub fn create<'a>(list_id: &'a str, store_id: &'a str) -> Self {
        RecipientType {
            list_id: Some(list_id.to_string()),
            segment_text: None,
            list_is_active: None,
            list_name: None,
            recipient_count: None,
            segment_opts: None,
            store_id: Some(store_id.to_string()),
        }
    }
}

// ============ Salesforce CRM Tracking ==============
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SalesforceCRMTrackingType {
    /// Create a campaign in a connected Salesforce account.
    #[serde(default)]
    pub campaign: bool,
    /// Update contact notes for a campaign based on a subscriber’s email address.
    #[serde(default)]
    pub notes: bool,
}

impl Default for SalesforceCRMTrackingType {
    fn default() -> Self {
        SalesforceCRMTrackingType {
            campaign: false,
            notes: false,
        }
    }
}

// ============ Capsule CRM Tracking ==============
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CapsuleCRMTrackingType {
    /// Update contact notes for a campaign based on a subscriber’s email addresses.
    #[serde(default)]
    pub notes: bool,
}

impl Default for CapsuleCRMTrackingType {
    fn default() -> Self {
        CapsuleCRMTrackingType { notes: false }
    }
}

// ============ Campaign Report Summary ==============
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CampaignReportSummaryType {
    /// The total number of opens for a campaign.
    #[serde(default)]
    pub opens: u64,
    /// The number of unique opens.
    #[serde(default)]
    pub unique_opens: u64,
    /// The number of unique opens divided by the total number of successful deliveries.
    #[serde(default)]
    pub open_rate: f32,
    /// The total number of clicks for an campaign.
    #[serde(default)]
    pub clicks: u64,
    /// The number of unique clicks.
    #[serde(default)]
    pub subscriber_clicks: u64,
    /// The number of unique clicks, divided by the total number of successful deliveries.
    #[serde(default)]
    pub click_rate: f32,
    /// E-Commerce stats for a campaign.
    #[serde(default)]
    pub ecommerce: Option<ECommerceReportType>,
}

impl Default for CampaignReportSummaryType {
    fn default() -> Self {
        CampaignReportSummaryType {
            opens: 0,
            unique_opens: 0,
            open_rate: 0.0,
            clicks: 0,
            subscriber_clicks: 0,
            click_rate: 0.0,
            ecommerce: None,
        }
    }
}

// ============ Automation Trigger ==============
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AutomationTriggerType {
    /// The type of Automation workflow.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workflow_type: Option<String>,
}

impl Default for AutomationTriggerType {
    fn default() -> Self {
        AutomationTriggerType {
            workflow_type: None,
        }
    }
}

impl AutomationTriggerType {
    ///
    /// Shortcut para el proceso de creación de una automatización
    ///
    /// Argumentos:
    ///     workflow_type: The type of Automation workflow. Currently only supports ‘abandonedCart’.
    ///
    pub fn create<'a>(workflow_type: &'a str) -> Self {
        AutomationTriggerType {
            workflow_type: Some(workflow_type.to_string()),
        }
    }
}

// ============ Automation Campaign Settings ==============
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AutomationCampaignSettingsType {
    /// The subject line for the campaign.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject_line: Option<String>,
    /// The preview text for the campaign.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preview_text: Option<String>,
    /// The title of the Automation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Desc: The ‘from’ name for the Automation (not an email address).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from_name: Option<String>,
    /// Desc: The reply-to email address for the Automation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<String>,
    /// Desc: Whether Mailchimp authenticated the Automation. Defaults to true.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authenticate: Option<bool>,
    /// Desc: Whether to use Mailchimp’s Conversations feature to manage out-of-office replies.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub use_conversation: Option<bool>,
    /// Desc: The Automation’s custom ‘To’ name, typically the first name merge field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub to_name: Option<String>,
    /// Desc: If the campaign is listed in a folder, the id for that folder.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub folder_id: Option<String>,
    /// Desc: Whether to automatically append Mailchimp’s default footer to the Automation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auto_footer: Option<bool>,
    /// Desc: Whether to automatically inline the CSS included with the Automation content.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inline_css: Option<bool>,
    /// Automatically tweet a link to the campaign archive page when the campaign is sent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auto_tweet: Option<bool>,
    /// An array of Facebook page ids to auto-post to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auto_fb_post: Option<Vec<u64>>,
    /// Allows Facebook comments on the campaign (also force-enables the Campaign Archive toolbar). Defaults to true.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fb_comments: Option<bool>,
    /// Send this campaign using Timewarp.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timewarp: Option<bool>,
    /// Allows Facebook comments on the campaign (also force-enables the Campaign Archive toolbar). Defaults to true.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template_id: Option<u64>,
    /// Whether the campaign uses the drag-and-drop editor.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub drag_and_drop: Option<bool>,
}

pub type CampaignSettingsType = AutomationCampaignSettingsType;

impl Default for AutomationCampaignSettingsType {
    fn default() -> Self {
        AutomationCampaignSettingsType {
            subject_line: None,
            preview_text: None,
            title: None,
            from_name: None,
            reply_to: None,
            use_conversation: None,
            to_name: None,
            folder_id: None,
            authenticate: None,
            auto_footer: None,
            inline_css: None,
            auto_tweet: None,
            auto_fb_post: None,
            fb_comments: None,
            timewarp: None,
            template_id: None,
            drag_and_drop: None,
        }
    }
}

impl AutomationCampaignSettingsType {
    ///
    /// Shortcut para el proceso de creación de una automatización
    ///
    /// Argumentos:
    ///     from_name: El ‘from’ para la automatización.
    ///     reply_to: La dirección de corrreo para la automatización, reply-to.
    ///
    pub fn create<'a>(from_name: &'a str, reply_to: &'a str) -> Self {
        AutomationCampaignSettingsType {
            subject_line: None,
            preview_text: None,
            title: None,
            from_name: Some(from_name.to_string()),
            folder_id: None,
            reply_to: Some(reply_to.to_string()),
            use_conversation: None,
            to_name: None,
            authenticate: None,
            auto_footer: None,
            inline_css: None,
            auto_tweet: None,
            auto_fb_post: None,
            fb_comments: None,
            timewarp: None,
            template_id: None,
            drag_and_drop: None,
        }
    }
    ///
    /// Shortcut para el proceso de creación de una automatización
    ///
    /// Argumentos:
    ///     title: Titulo de la automatizacion
    ///     from_name: El ‘from’ para la automatización.
    ///     reply_to: La dirección de corrreo para la automatización, reply-to.
    ///
    pub fn update<'a>(title: &'a str, from_name: &'a str, reply_to: &'a str) -> Self {
        AutomationCampaignSettingsType {
            subject_line: None,
            preview_text: None,
            title: Some(title.to_string()),
            from_name: Some(from_name.to_string()),
            folder_id: None,
            reply_to: Some(reply_to.to_string()),
            use_conversation: None,
            to_name: None,
            authenticate: None,
            auto_footer: None,
            inline_css: None,
            auto_tweet: None,
            auto_fb_post: None,
            fb_comments: None,
            timewarp: None,
            template_id: None,
            drag_and_drop: None,
        }
    }
}

// ============ Automation Tracking Options ==============
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AutomationTrackingOptionsType {
    /// Whether to track opens. Defaults to true.
    #[serde(default)]
    pub opens: bool,
    /// Desc: Whether to track clicks in the HTML version of the Automation. Defaults to true.
    #[serde(default)]
    pub html_clicks: bool,
    /// Desc: Whether to track clicks in the plain-text version of the Automation. Defaults to true.
    #[serde(default)]
    pub text_clicks: bool,
    /// Desc: Whether to enable eCommerce360 tracking.
    #[serde(default)]
    pub goal_tracking: bool,
    /// Desc: The Automation’s custom ‘To’ name, typically the first name merge field.
    #[serde(default)]
    pub ecomm360: bool,
    /// Desc: The custom slug for Google Analytics tracking (max of 50 bytes).
    #[serde(default)]
    pub google_analytics: String,
    /// Desc: The custom slug for ClickTale tracking (max of 50 bytes).
    #[serde(default)]
    pub clicktale: String,
    /// Desc: Salesforce tracking options for an Automation. Must be using Mailchimp’s
    /// built-in Salesforce integration.
    #[serde(default)]
    pub salesforce: SalesforceCRMTrackingType,
    /// Desc: Capsule tracking options for an Automation. Must be using Mailchimp’s
    /// built-in Capsule integration.
    #[serde(default)]
    pub capsule: CapsuleCRMTrackingType,
}

impl Default for AutomationTrackingOptionsType {
    fn default() -> Self {
        AutomationTrackingOptionsType {
            opens: true,
            html_clicks: true,
            text_clicks: true,
            goal_tracking: false,
            ecomm360: false,
            google_analytics: "".to_string(),
            clicktale: "".to_string(),
            salesforce: SalesforceCRMTrackingType::default(),
            capsule: CapsuleCRMTrackingType::default(),
        }
    }
}

pub type CampaignTrackingOptionsType = AutomationTrackingOptionsType;

// ============ Automation Workflows ==============
// GET /automations/{workflow_id}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AutomationWorkflowType {
    /// A string that identifies the Automation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Desc: The date and time the Automation was created in ISO 8601 format.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,

    /// Desc: The date and time the Automation was started in ISO 8601 format.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,

    /// Desc: The current status of the Automation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    /// Desc: The total number of emails sent for the Automation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub emails_sent: Option<u64>,

    /// Desc: List settings for the Automation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recipients: Option<RecipientType>,

    /// Desc: The settings for the Automation workflow.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub settings: Option<AutomationCampaignSettingsType>,

    /// Desc: The tracking options for the Automation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tracking: Option<AutomationTrackingOptionsType>,

    /// Desc: Available triggers for Automation workflows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trigger_settings: Option<AutomationTriggerType>,

    /// Desc: A summary of opens, clicks, and unsubscribes for sent campaigns.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub report_summary: Option<CampaignReportSummaryType>,

    /// Desc: A list of link types and descriptions for the API schema documents.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub _links: Option<Vec<LinkType>>,
}

// ============ Authorized Apps ==============
// GET /automations
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AutomationsType {
    /// An array of objects, each representing an authorized application.
    #[serde(default)]
    pub automations: Vec<AutomationWorkflowType>,
    /// Desc: The total number of items matching the query regardless of pagination.
    #[serde(default)]
    pub total_items: u32,
    /// Desc: A list of link types and descriptions for the API schema documents.
    #[serde(default)]
    pub _links: Vec<LinkType>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AutomationModifier {
    /// The settings for the Automation workflow.
    #[serde(default)]
    pub settings: Option<AutomationCampaignSettingsType>,
    /// Desc: The total number of items matching the query regardless of pagination.
    #[serde(default)]
    pub delay: Option<AutomationDelayType>,
    /// Desc: A list of link types and descriptions for the API schema documents.
    #[serde(default)]
    pub recipients: Option<RecipientType>,
    /// Desc: A list of link types and descriptions for the API schema documents.
    #[serde(default)]
    pub trigger_settings: Option<AutomationTriggerType>,
}

// ============ Workflow Email ==============
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SocialCardType {
    /// The url for the header image for the card.
    #[serde(default)]
    pub image_url: Option<String>,
    /// A short summary of the campaign to display.
    #[serde(default)]
    pub description: Option<String>,
    /// The title for the card. Typically the subject line of the campaign.
    #[serde(default)]
    pub title: Option<String>,
}

impl Default for SocialCardType {
    fn default() -> Self {
        SocialCardType {
            image_url: None,
            description: None,
            title: None,
        }
    }
}

// ============ Workflow Email ==============
// GET /automations/{workflow_id}/emails/{workflow_email_id}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WorkflowEmailType {
    /// A string that uniquely identifies the Automation email.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The ID used in the Mailchimp web application. View this automation in your Mailchimp account at https://{dc}.admin.mailchimp.com/campaigns/show/?id={web_id}.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub web_id: Option<u64>,
    /// A string that uniquely identifies an Automation workflow.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workflow_id: Option<String>,
    /// The position of an Automation email in a workflow.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position: Option<u64>,
    /// The delay settings for an Automation email.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delay: Option<AutomationDelayType>,
    /// The date and time the campaign was created in ISO 8601 format.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// The date and time the campaign was started in ISO 8601 format.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// The link to the campaign’s archive version in ISO 8601 format.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub archive_url: Option<String>,
    /// The total number of emails sent for this campaign.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub emails_sent: Option<u64>,
    /// The date and time a campaign was sent in ISO 8601 format
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub send_time: Option<String>,
    /// How the campaign’s content is put together (‘template’, ‘drag_and_drop’, ‘html’, ‘url’).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    /// Determines if the automation email needs its blocks refreshed by opening the web-based campaign editor.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub needs_block_refresh: Option<bool>,
    /// Determines if the campaign contains the |BRAND:LOGO| merge tag.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_logo_merge_tag: Option<bool>,
    /// List settings for the campaign.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recipients: Option<RecipientType>,
    /// Settings for the campaign including the email subject, from name, and from email address.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub settings: Option<AutomationCampaignSettingsType>,
    /// The tracking options for a campaign.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tracking: Option<AutomationTrackingOptionsType>,
    /// The preview for the campaign, rendered by social networks like Facebook and Twitter. Learn more.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub social_card: Option<SocialCardType>,
    /// Available triggers for Automation workflows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trigger_settings: Option<AutomationTriggerType>,
    /// For sent campaigns, a summary of opens, clicks, and unsubscribes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub report_summary: Option<CampaignReportSummaryType>,
    /// A list of link types and descriptions for the API schema documents.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub _links: Option<Vec<LinkType>>,
}

// GET /automations/{workflow_id}/emails
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WorkflowEmailsType {
    /// An array of objects, each representing an email in an Automation workflow.
    #[serde(default)]
    pub emails: Vec<WorkflowEmailType>,
    /// Desc: The total number of items matching the query regardless of pagination.
    #[serde(default)]
    pub total_items: u32,
    /// Desc: A list of link types and descriptions for the API schema documents.
    #[serde(default)]
    pub _links: Vec<LinkType>,
}
