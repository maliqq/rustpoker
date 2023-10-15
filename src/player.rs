use crate::locale::Country;

#[derive(Debug, Copy, Clone)]
pub struct Avatar<'a> {
    url: &'a str,
}

#[derive(Debug, Clone)]
pub struct Player<'a> {
    pub name: &'a str,
    avatar: Option<Avatar<'a>>,
    country: Option<Country>,
}

impl<'a> Player<'a> {
    pub fn blank_player(name: &'a str) -> Self {
        Player {
            name,
            avatar: None,
            country: None,
        }
    }
}
