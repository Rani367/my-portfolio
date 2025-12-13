//! Social media links configuration data.

/// Represents a social media link.
#[derive(Clone, Debug, PartialEq)]
pub struct Social {
    pub id: u32,
    pub text: &'static str,
    pub icon: &'static str,
    pub bg: &'static str,
    pub link: &'static str,
}

/// All social media links for the contact window.
pub const SOCIALS: &[Social] = &[
    Social {
        id: 1,
        text: "Github",
        icon: "/public/icons/github.svg",
        bg: "#f4656b",
        link: "https://github.com/Rani367",
    },
];
