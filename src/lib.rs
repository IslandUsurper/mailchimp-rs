//! Mailchimp API

#![deny(
    bad_style,
    missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications
)]
#![cfg_attr(not(debug_assertions), deny(warnings))]

mod api;
mod api_root;
mod authorized_apps;
mod automations;
mod campaigns;
mod conversations;
mod internal;
pub mod iter;
mod landing_pages;
mod list;
mod reports;
pub mod types;

pub use crate::api::MailchimpApi;
pub use crate::api_root::ApiRoot;
pub use crate::authorized_apps::{AuthorizedApps, AuthorizedFilter};
pub use crate::automations::{Automations, AutomationsFilter};
pub use crate::campaigns::{CampaignFilter, Campaigns};
pub use crate::conversations::Conversations;
pub use crate::internal::error_type::MailchimpErrorType;
pub use crate::landing_pages::LandingPages;
pub use crate::list::{ListFilter, Lists};
pub use crate::reports::Reports;
