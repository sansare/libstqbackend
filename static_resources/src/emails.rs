use std::fmt;
use std::str::FromStr;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SimpleMail {
    pub to: String,
    pub subject: String,
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmailUser {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
}

pub trait Email {
    fn into_send_mail(self) -> SimpleMail;
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrderUpdateStateForUser {
    pub user: EmailUser,
    pub order_slug: String,
    pub order_state: String,
    pub cluster_url: String,
}

impl Email for OrderUpdateStateForUser {
    fn into_send_mail(self) -> SimpleMail {
        SimpleMail {
            to : self.user.email,
            subject : format!("The order {} status", self.order_slug),
            text : format!(
                "Orders' {} state is '{}' now. You can view current info about your order on <a href=\"{}/profile/orders/{}\">this page</a>.",
                self.order_slug, self.order_state, self.cluster_url, self.order_slug
            ),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrderUpdateStateForStore {
    pub store_email: String,
    pub order_slug: String,
    pub order_state: String,
    pub cluster_url: String,
    pub store_id: String,
}

impl Email for OrderUpdateStateForStore {
    fn into_send_mail(self) -> SimpleMail {
        SimpleMail {
            to: self.store_email,
            subject: format!("The order {} status", self.order_slug),
            text: format!(
                "Orders' {} state is '{}' now. You can view current order info on <a href=\"{}/manage/store/{}/orders/{}\">this page</a>.",
                self.order_slug, self.order_state, self.cluster_url, self.store_id, self.order_slug
            ),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrderCreateForUser {
    pub user: EmailUser,
    pub order_slug: String,
    pub cluster_url: String,
}

impl Email for OrderCreateForUser {
    fn into_send_mail(self) -> SimpleMail {
        SimpleMail {
            to: self.user.email,
            subject: format!("New order {}.", self.order_slug),
            text: format!(
                "Order {} was created. You can view current info about your order on <a href=\"{}/profile/orders/{}\">this page</a>.",
                self.order_slug, self.cluster_url, self.order_slug
            ),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrderCreateForStore {
    pub store_email: String,
    pub order_slug: String,
    pub cluster_url: String,
    pub store_id: String,
}

impl Email for OrderCreateForStore {
    fn into_send_mail(self) -> SimpleMail {
        SimpleMail {
            to: self.store_email,
            subject: format!("New order {}.", self.order_slug),
            text: format!(
                "Order {} was created. You can view current order info on <a href=\"{}/manage/store/{}/orders/{}\">this page</a>.",
                self.order_slug, self.cluster_url, self.store_id, self.order_slug
            ),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmailVerificationForUser {
    pub user: EmailUser,
    pub verify_email_path: String,
    pub token: String,
}

impl Email for EmailVerificationForUser {
    fn into_send_mail(self) -> SimpleMail {
        SimpleMail {
            to: self.user.email,
            subject: "Verify your account on Storiqa".to_string(),
            text: format!("{}/{}", self.verify_email_path, self.token),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PasswordResetForUser {
    pub user: EmailUser,
    pub reset_password_path: String,
    pub token: String,
}

impl Email for PasswordResetForUser {
    fn into_send_mail(self) -> SimpleMail {
        SimpleMail {
            to: self.user.email,
            subject: "Password reset".to_string(),
            text: format!("{}/{}", self.reset_password_path, self.token),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApplyPasswordResetForUser {
    pub user: EmailUser,
    pub cluster_url: String,
}

impl Email for ApplyPasswordResetForUser {
    fn into_send_mail(self) -> SimpleMail {
        SimpleMail {
            to: self.user.email,
            subject: "Successful password reset".to_string(),
            text: "Password for linked account has been successfully reset.".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApplyEmailVerificationForUser {
    pub user: EmailUser,
}

impl Email for ApplyEmailVerificationForUser {
    fn into_send_mail(self) -> SimpleMail {
        SimpleMail {
            to: self.user.email,
            subject: "Successful registration".to_string(),
            text: "Email for linked account has been verified.".to_string(),
        }
    }
}

#[derive(GraphQLEnum, Deserialize, Serialize, Clone, Copy, PartialEq, Eq, Debug, DieselTypes)]
#[graphql(name = "TemplateVariant", description = "Template variant")]
pub enum TemplateVariant {
    #[graphql(description = "order create for user.")]
    OrderCreateForUser,
    #[graphql(description = "order update state for user.")]
    OrderUpdateStateForUser,
    #[graphql(description = "order create for store.")]
    OrderCreateForStore,
    #[graphql(description = "order update state for store.")]
    OrderUpdateStateForStore,
    #[graphql(description = "email verification.")]
    EmailVerificationForUser,
    #[graphql(description = "password reset.")]
    PasswordResetForUser,
    #[graphql(description = "apply password reset.")]
    ApplyPasswordResetForUser,
    #[graphql(description = "apply email verification.")]
    ApplyEmailVerificationForUser,
}

impl FromStr for TemplateVariant {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "order_create_for_user" => Ok(TemplateVariant::OrderCreateForUser),
            "order_update_state_for_user" => Ok(TemplateVariant::OrderUpdateStateForUser),
            "order_create_for_store" => Ok(TemplateVariant::OrderCreateForStore),
            "order_update_state_for_store" => Ok(TemplateVariant::OrderUpdateStateForStore),
            "email_verification_for_user" => Ok(TemplateVariant::EmailVerificationForUser),
            "password_reset_for_user" => Ok(TemplateVariant::PasswordResetForUser),
            "apply_password_reset_for_user" => Ok(TemplateVariant::ApplyPasswordResetForUser),
            "apply_email_verification_for_user" => Ok(TemplateVariant::ApplyEmailVerificationForUser),
            _ => Err(()),
        }
    }
}

impl fmt::Display for TemplateVariant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TemplateVariant::OrderCreateForUser => write!(f, "order_create_for_user"),
            TemplateVariant::OrderUpdateStateForUser => write!(f, "order_update_state_for_user"),
            TemplateVariant::OrderCreateForStore => write!(f, "order_create_for_store"),
            TemplateVariant::OrderUpdateStateForStore => write!(f, "order_update_state_for_store"),
            TemplateVariant::EmailVerificationForUser => write!(f, "email_verification_for_user"),
            TemplateVariant::PasswordResetForUser => write!(f, "password_reset_for_user"),
            TemplateVariant::ApplyPasswordResetForUser => write!(f, "apply_password_reset_for_user"),
            TemplateVariant::ApplyEmailVerificationForUser => write!(f, "apply_email_verification_for_user"),
        }
    }
}
