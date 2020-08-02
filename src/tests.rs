use super::{Command, Filter};

#[inline]
fn verify(left: impl AsRef<str>, right: &str) {
    assert_eq!(left.as_ref(), right)
}

#[test]
fn filter_single_criteria() {
    verify(Filter::new().workspace("c"), "[workspace=c]")
}

#[test]
fn filter_multiple_criteria() {
    verify(
        Filter::new().pid("1").shell("xwayland"),
        "[pid=1 shell=xwayland]",
    )
}

#[test]
fn border_csd() {
    verify(Command::new().border().csd(), "border csd")
}

#[test]
fn border_none() {
    verify(Command::new().border().none(), "border none")
}

#[test]
fn border_toggle() {
    verify(Command::new().border().toggle(), "border toggle")
}

#[test]
fn border_normal() {
    verify(Command::new().border().normal(), "border normal")
}

#[test]
fn border_pixel() {
    verify(Command::new().border().pixel(), "border pixel")
}

#[test]
fn border_normal_with() {
    verify(
        Command::new().border().normal().with(10),
        "border normal 10",
    )
}

#[test]
fn border_pixel_with() {
    verify(Command::new().border().pixel().with(10), "border pixel 10")
}

#[test]
fn exit() {
    verify(Command::new().exit(), "exit")
}

#[test]
fn floating_enable() {
    verify(Command::new().floating().enable(), "floating enable")
}

#[test]
fn floating_disable() {
    verify(Command::new().floating().disable(), "floating disable")
}

#[test]
fn floating_toggle() {
    verify(Command::new().floating().toggle(), "floating toggle")
}

#[test]
fn fullscreen_enable() {
    verify(Command::new().fullscreen().enable(), "fullscreen enable")
}

#[test]
fn fullscreen_disable() {
    verify(Command::new().fullscreen().disable(), "fullscreen disable")
}

#[test]
fn fullscreen_toggle() {
    verify(Command::new().fullscreen().toggle(), "fullscreen toggle")
}

#[test]
fn gaps_inner_all_set() {
    verify(
        Command::new().gaps().inner().all().set().amount(1),
        "gaps inner all set 1",
    )
}

#[test]
fn gaps_outer_current_plus() {
    verify(
        Command::new().gaps().outer().current().plus().amount(1),
        "gaps outer current plus 1",
    )
}

#[test]
fn gaps_horizontal_all_minus() {
    verify(
        Command::new().gaps().horizontal().all().minus().amount(1),
        "gaps horizontal all minus 1",
    )
}

#[test]
fn inhibit_idle_focus() {
    verify(Command::new().inhibit_idle().focus(), "inhibit_idle focus")
}

#[test]
fn inhibit_idle_fullscreen() {
    verify(
        Command::new().inhibit_idle().fullscreen(),
        "inhibit_idle fullscreen",
    )
}

#[test]
fn inhibit_idle_open() {
    verify(Command::new().inhibit_idle().open(), "inhibit_idle open")
}

#[test]
fn inhibit_idle_none() {
    verify(Command::new().inhibit_idle().none(), "inhibit_idle none")
}

#[test]
fn inhibit_idle_visible() {
    verify(
        Command::new().inhibit_idle().visible(),
        "inhibit_idle visible",
    )
}
