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
        link: "https://github.com/JavaScript-Mastery-Pro",
    },
    Social {
        id: 2,
        text: "Platform",
        icon: "/public/icons/atom.svg",
        bg: "#4bcb63",
        link: "https://jsmastery.com/",
    },
    Social {
        id: 3,
        text: "Twitter/X",
        icon: "/public/icons/twitter.svg",
        bg: "#ff866b",
        link: "https://x.com/jsmasterypro",
    },
    Social {
        id: 4,
        text: "LinkedIn",
        icon: "/public/icons/linkedin.svg",
        bg: "#05b6f6",
        link: "https://www.linkedin.com/company/javascriptmastery/posts/?feedView=all",
    },
];
