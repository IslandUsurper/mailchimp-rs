use super::automation_campaign::{
    CampaignReportSummaryType, CampaignSettingsType, CampaignTrackingOptionsType, RecipientType,
    SocialCardType,
};
use super::link::LinkType;
use crate::api::{MailchimpApi, MailchimpApiUpdate};
use crate::iter::MailchimpCollection;

///
/// The days of the week to send a daily RSS Campaign.
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CampaignDeliveryStatusType {
    /// Whether Campaign Delivery Status is enabled for this account and campaign.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// Whether a campaign send can be canceled.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_cancel: Option<bool>,
    /// The current state of a campaign delivery.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// The total number of emails confirmed sent for this campaign so far.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub emails_sent: Option<u64>,
    /// The total number of emails canceled for this campaign.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub emails_canceled: Option<u64>,
}

impl Default for CampaignDeliveryStatusType {
    fn default() -> Self {
        CampaignDeliveryStatusType {
            enabled: None,
            can_cancel: None,
            status: None,
            emails_sent: None,
            emails_canceled: None,
        }
    }
}
///
/// The days of the week to send a daily RSS Campaign.
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ABTestingOptionsType {
    /// The type of AB split to run.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub split_test: Option<String>,
    /// How we should evaluate a winner. Based on ‘opens’, ‘clicks’, or ‘manual’.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pick_winner: Option<String>,
    /// How unit of time for measuring the winner (‘hours’ or ‘days’).
    /// This cannot be changed after a campaign is sent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wait_units: Option<String>,
    /// The amount of time to wait before picking a winner. This cannot be
    /// changed after a campaign is sent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wait_time: Option<u64>,
    /// The size of the split groups. Campaigns split based on ‘schedule’
    /// are forced to have a 50⁄50 split. Valid split integers are between 1-50.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub split_size: Option<u64>,
    /// For campaigns split on ‘From Name’, the name for Group A.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from_name_a: Option<String>,
    /// For campaigns split on ‘From Name’, the name for Group B.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from_name_b: Option<String>,
    /// For campaigns split on ‘From Name’, the reply-to address for Group A.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reply_email_a: Option<String>,
    /// For campaigns split on ‘From Name’, the reply-to address for Group B.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reply_email_b: Option<String>,
    /// For campaigns split on ‘Subject Line’, the subject line for Group A.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject_a: Option<String>,
    /// For campaigns split on ‘Subject Line’, the subject line for Group B.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject_b: Option<String>,
    /// The send time for Group A.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub send_time_a: Option<String>,
    /// The send time for Group B.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub send_time_b: Option<String>,
    /// The send time for the winning version.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub send_time_winner: Option<String>,
}

impl Default for ABTestingOptionsType {
    fn default() -> Self {
        ABTestingOptionsType {
            split_test: None,
            pick_winner: None,
            wait_units: None,
            wait_time: None,
            split_size: None,
            from_name_a: None,
            from_name_b: None,
            reply_email_a: None,
            reply_email_b: None,
            subject_a: None,
            subject_b: None,
            send_time_a: None,
            send_time_b: None,
            send_time_winner: None,
        }
    }
}

///
/// The days of the week to send a daily RSS Campaign.
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DailySendingDaysType {
    /// Sends the daily RSS Campaign on Sundays.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sunday: Option<bool>,
    /// Sends the daily RSS Campaign on Mondays.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub monday: Option<bool>,
    /// Sends the daily RSS Campaign on Tuesdays.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tuesday: Option<bool>,
    /// Sends the daily RSS Campaign on Wednesdays.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wednesday: Option<bool>,
    /// Sends the daily RSS Campaign on Thursdays.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thursday: Option<bool>,
    /// Sends the daily RSS Campaign on Fridays.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub friday: Option<bool>,
    /// Sends the daily RSS Campaign on Saturdays.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub saturday: Option<bool>,
}

impl Default for DailySendingDaysType {
    fn default() -> Self {
        DailySendingDaysType {
            sunday: None,
            monday: None,
            tuesday: None,
            wednesday: None,
            thursday: None,
            friday: None,
            saturday: None,
        }
    }
}

///
/// The schedule for sending the RSS Campaign.
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SendingScheduleType {
    /// The hour to send the campaign in local time. Acceptable hours are 0-23.
    /// For example, ‘4’ would be 4am in your account’s default time zone.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hour: Option<u64>,
    /// The days of the week to send a daily RSS Campaign.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub daily_send: Option<DailySendingDaysType>,
    /// The day of the week to send a weekly RSS Campaign.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weekly_send_day: Option<String>,
    /// The day of the month to send a monthly RSS Campaign. Acceptable days are 0-31,
    /// where ‘0’ is always the last day of a month. Months with fewer than the
    /// selected number of days will not have an RSS campaign sent out that day.
    /// For example, RSS Campaigns set to send on the 30th will not go out in February.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub monthly_send_date: Option<f32>,
}

impl Default for SendingScheduleType {
    fn default() -> Self {
        SendingScheduleType {
            hour: None,
            daily_send: None,
            weekly_send_day: None,
            monthly_send_date: None,
        }
    }
}

///
/// RSS options for a campaign.
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RSSOptionsType {
    /// The URL for the RSS feed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub feed_url: Option<String>,
    /// The frequency of the RSS Campaign.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frequency: Option<String>,
    /// The schedule for sending the RSS Campaign.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schedule: Option<SendingScheduleType>,
    /// The date the campaign was last sent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_sent: Option<String>,
    /// Whether to add CSS to images in the RSS feed to constrain their width in campaigns.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub constrain_rss_img: Option<bool>,
}

impl Default for RSSOptionsType {
    fn default() -> Self {
        RSSOptionsType {
            feed_url: None,
            frequency: None,
            schedule: None,
            last_sent: None,
            constrain_rss_img: None,
        }
    }
}

///
/// The settings specific to A/B test campaigns.
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CombinationsType {
    /// Unique ID for the combination.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The index of variate_settings.subject_lines used.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject_line: Option<u64>,
    /// The index of variate_settings.send_times used.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub send_time: Option<u64>,
    /// The index of variate_settings.from_names used.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from_name: Option<u64>,
    /// The index of variate_settings.reply_to_addresses used.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<u64>,
    /// The index of variate_settings.contents used.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content_description: Option<u64>,
    /// The number of recipients for this combination.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recipients: Option<u64>,
}

impl Default for CombinationsType {
    fn default() -> Self {
        CombinationsType {
            id: None,
            subject_line: None,
            send_time: None,
            from_name: None,
            reply_to: None,
            content_description: None,
            recipients: None,
        }
    }
}

///
/// The settings specific to A/B test campaigns.
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VariateSettingsType {
    /// ID for the winning combination.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub winning_combination_id: Option<String>,
    /// ID of the campaign that was sent to the remaining recipients
    /// based on the winning combination.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub winning_campaign_id: Option<String>,
    /// The combination that performs the best. This may be determined
    /// automatically by click rate, open rate, or total revenue—or you
    /// may choose manually based on the reporting data you find the most
    /// valuable. For Multivariate Campaigns testing send_time,
    ///  winner_criteria is ignored. For Multivariate Campaigns with ‘manual’
    ///  as the winner_criteria, the winner must be chosen in the Mailchimp
    /// web application.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub winner_criteria: Option<String>,
    /// The number of minutes to wait before choosing the winning campaign.
    /// The value of wait_time must be greater than 0 and in whole hours,
    /// specified in minutes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wait_time: Option<u64>,
    /// The percentage of recipients to send the test combinations to,
    /// must be a value between 10 and 100.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub test_size: Option<u64>,
    /// The possible subject lines to test. If no subject lines are provided,
    /// settings.subject_line will be used.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject_lines: Option<Vec<String>>,
    /// The possible send times to test. The times provided should
    /// be in the format YYYY-MM-DD HH:MM:SS. If send_times are provided to
    /// test, the test_size will be set to 100% and winner_criteria will be ignored.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub send_times: Option<Vec<String>>,
    /// The possible from names. The number of from_names provided must match the number
    /// of reply_to_addresses. If no from_names are provided, settings.from_name will be used.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from_names: Option<Vec<String>>,
    /// The possible reply-to addresses. The number of reply_to_addresses provided must match
    /// the number of from_names. If no reply_to_addresses are provided, settings.reply_to will be used.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reply_to_addresses: Option<Vec<String>>,
    /// Descriptions of possible email contents. To set campaign contents,
    /// make a PUT request to /campaigns/{campaign_id}/content with the field ‘variate_contents’.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contents: Option<Vec<String>>,
    /// Combinations of possible variables used to build emails.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub combinations: Option<Vec<CombinationsType>>,
}

impl Default for VariateSettingsType {
    fn default() -> Self {
        VariateSettingsType {
            winning_combination_id: None,
            winning_campaign_id: None,
            winner_criteria: None,
            wait_time: None,
            test_size: None,
            subject_lines: None,
            send_times: None,
            from_names: None,
            reply_to_addresses: None,
            contents: None,
            combinations: None,
        }
    }
}

// ============ Campaign ==============
// GET /campaigns/{campaign_id}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CampaignType {
    /// A string that uniquely identifies this campaign.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The ID used in the Mailchimp web application. View this campaign in
    /// your Mailchimp account at https://{dc}.admin.mailchimp.com/campaigns/show/?id={web_id}.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub web_id: Option<u64>,
    /// If this campaign is the child of another campaign, this identifies the parent
    /// campaign. For Example, for RSS or Automation children.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_campaign_id: Option<String>,
    /// There are four types of campaigns you can create in Mailchimp. A/B Split
    /// campaigns have been deprecated and variate campaigns should be used instead.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub campaign_type: Option<String>,
    /// The date and time the campaign was created in ISO 8601 format.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// The link to the campaign’s archive version in ISO 8601 format.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub archive_url: Option<String>,
    /// The original link to the campaign’s archive version.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub long_archive_url: Option<String>,
    /// The current status of the campaign.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// The total number of emails sent for this campaign.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub emails_sent: Option<u64>,
    /// The date and time a campaign was sent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub send_time: Option<String>,
    /// How the campaign’s content is put together (‘template’, ‘drag_and_drop’, ‘html’, ‘url’).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    /// Determines if the campaign needs its blocks refreshed by opening the web-based campaign editor.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub needs_block_refresh: Option<bool>,
    /// Determines if the campaign contains the |BRAND:LOGO| merge tag.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_logo_merge_tag: Option<bool>,
    /// Determines if the campaign qualifies to be resent to non-openers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resendable: Option<bool>,
    /// List settings for the campaign.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recipients: Option<RecipientType>,
    /// The settings for your campaign, including subject, from name,
    /// reply-to address, and more.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub settings: Option<CampaignSettingsType>,
    /// The settings specific to A/B test campaigns.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variate_settings: Option<VariateSettingsType>,
    /// The tracking options for a campaign.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tracking: Option<CampaignTrackingOptionsType>,
    /// RSS options for a campaign.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rss_opts: Option<RSSOptionsType>,
    /// A/B Testing options for a campaign.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ab_split_opts: Option<ABTestingOptionsType>,
    /// The preview for the campaign, rendered by social networks like
    /// Facebook and Twitter. Learn more.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub social_card: Option<SocialCardType>,
    /// For sent campaigns, a summary of opens, clicks, and e-commerce data.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub report_summary: Option<CampaignReportSummaryType>,
    /// Updates on campaigns in the process of sending.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delivery_status: Option<CampaignDeliveryStatusType>,
    /// Desc: A list of link types and descriptions for the API schema documents.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub _links: Option<Vec<LinkType>>,

    // Mailchimp API
    #[serde(skip)]
    _api: MailchimpApi,
}

impl MailchimpApiUpdate for CampaignType {
    /**
     * Update API
     */
    fn set_api(&mut self, n_api: &MailchimpApi) {
        self._api = n_api.clone()
    }
}

// GET /campaigns
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CampaignsType {
    /// An array of objects, each representing an email in an Automation workflow.
    #[serde(default)]
    pub campaigns: Vec<CampaignType>,
    /// Desc: The total number of items matching the query regardless of pagination.
    #[serde(default)]
    pub total_items: u32,
    /// Desc: A list of link types and descriptions for the API schema documents.
    #[serde(default)]
    pub _links: Vec<LinkType>,
}

impl MailchimpCollection<CampaignType> for CampaignsType {
    /// Total Items
    fn get_total_items(&self) -> u32 {
        self.total_items
    }
    /// Data
    fn get_values(&self) -> Vec<CampaignType> {
        self.campaigns.clone()
    }
}

impl Default for CampaignsType {
    fn default() -> Self {
        CampaignsType {
            campaigns: Vec::new(),
            total_items: 0,
            _links: Vec::new(),
        }
    }
}