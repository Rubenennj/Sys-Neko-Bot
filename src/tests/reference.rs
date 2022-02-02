use serenity::cache::{Cache, CacheUpdate};
use serenity::model::user::User;

fn as_ref(i: &'static dyn AsRef<str>) {}

#[test]
fn reference() {
    let g = &10;
}
