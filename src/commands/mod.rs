use super::command::{AddFilter, Chained, Command, Finalize};
use super::states::*;

mod assign;
mod border;
mod default_border;
mod floating;
mod floating_modifier;
mod floating_size;
mod focus;
mod focus_follows_mouse;
mod focus_on_window_activation;
mod focus_wrapping;
mod fullscreen;
mod gaps;
mod hide_edge_borders;
mod inhibit_idle;
mod layout;
mod mark;
mod max_render_time;
mod mouse_wraping;
mod opacity;
mod popup_during_fullscreen;
mod rename;
mod resize;
mod scratchpad;
mod shortcuts_inhibitor;
mod show_marks;
mod smart_borders;
mod smart_gaps;
mod split;
mod sticky;
mod swap;
mod sway_move;
mod client;
mod tiling_drag;
mod title_align;
mod titlebar_padding;
mod unbindswitch;
mod unmark;
mod urgent;
mod workspace;
mod workspace_auto_back_and_forth;

impl<T: Chained> Command<T> {
    pub fn for_window(self, criteria: impl AsRef<str>) -> Command<ForWindow> {
        self.push_str("for_window").push_str(criteria).transmute()
    }
}

impl<T: AddFilter> Command<T> {
    pub fn filter(self, criteria: impl AsRef<str>) -> Command<Filter> {
        self.push_str(criteria).transmute()
    }
}

impl<T: Finalize> Command<T> {
    //TODO: bar

    pub fn border(self) -> Command<Border> {
        self.push_str("border").transmute()
    }

    //TODO: create_output

    pub fn exit(self) -> Command<Valid> {
        self.push_str("exit").transmute()
    }

    pub fn floating(self) -> Command<Floating> {
        self.push_str("floating").transmute()
    }

    pub fn focus(self) -> Command<Valid<Focus>> {
        self.push_str("focus").transmute()
    }

    //TODO: force_display_urgency_hint

    pub fn fullscreen(self) -> Command<Fullscreen> {
        self.push_str("fullscreen").transmute()
    }

    pub fn gaps(self) -> Command<Gaps> {
        self.push_str("gaps").transmute()
    }

    pub fn inhibit_idle(self) -> Command<InhibitIdle> {
        self.push_str("inhibit_idle").transmute()
    }

    pub fn layout(self) -> Command<Layout> {
        self.push_str("layout").transmute()
    }

    pub fn max_render_time(self) -> Command<MaxRenderTime> {
        self.push_str("max_render_time").transmute()
    }

    pub fn sway_move(self) -> Command<Move> {
        self.push_str("move").transmute()
    }

    pub fn nop(self) -> Command<Valid> {
        self.push_str("nop").transmute()
    }

    pub fn reload(self) -> Command<Valid> {
        self.push_str("reload").transmute()
    }

    pub fn rename(self) -> Command<Rename> {
        self.push_str("rename").transmute()
    }

    pub fn resize(self) -> Command<Resize> {
        self.push_str("resize").transmute()
    }

    pub fn scratchpad(self) -> Command<Scratchpad> {
        self.push_str("scratchpad").transmute()
    }

    pub fn shortcuts_inhibitor(self) -> Command<ShortcutsInhibitor> {
        self.push_str("shortcuts_inhibitor").transmute()
    }

    pub fn split(self) -> Command<Split> {
        self.push_str("split").transmute()
    }

    pub fn sticky(self) -> Command<Sticky> {
        self.push_str("sticky").transmute()
    }

    pub fn swap(self) -> Command<Swap> {
        self.push_str("swap").transmute()
    }

    pub fn title_format(self, title_format: impl AsRef<str>) -> Command<Valid> {
        self.push_str("title_format")
            .push_str(title_format)
            .transmute()
    }

    pub fn assign(self, criteria: impl AsRef<str>) -> Command<Assign> {
        self.push_str("assign").push_str(criteria).transmute()
    }

    //TODO: bindsym

    //TODO: bindcode

    //TODO: bindswitch

    pub fn client(self) -> Command<Client> {
        self.push_str("client").push_char('.').transmute()
    }

    pub fn default_border(self) -> Command<DefaultBorder> {
        self.push_str("default_border").transmute()
    }

    pub fn default_floating_border(self) -> Command<DefaultBorder> {
        self.push_str("default_floating_border").transmute()
    }

    pub fn exec(self, exec: impl AsRef<str>) -> Command<Valid> {
        self.push_str("exec").push_str(exec).transmute()
    }

    pub fn exec_always(self, exec_always: impl AsRef<str>) -> Command<Valid> {
        self.push_str("exec_always")
            .push_str(exec_always)
            .transmute()
    }

    pub fn floating_maximum_size(self) -> Command<FloatingSize> {
        self.push_str("floating_maximum_size").transmute()
    }

    pub fn floating_minimum_size(self) -> Command<FloatingSize> {
        self.push_str("floating_minimum_size").transmute()
    }

    pub fn floating_modifier(self) -> Command<FloatingModifier> {
        self.push_str("floating_modifier").transmute()
    }

    pub fn focus_follows_mouse(self) -> Command<FocusFollowMouse> {
        self.push_str("focus_follows_mouse").transmute()
    }

    pub fn focus_on_window_activation(self) -> Command<FocusOnWindowActivation> {
        self.push_str("focus_on_window_activation").transmute()
    }

    pub fn focus_wrapping(self) -> Command<FocusWrapping> {
        self.push_str("focus_wrapping").transmute()
    }

    //TODO: font

    pub fn titlebar_border_thickness(self, px: usize) -> Command<Valid> {
        self.push_str("titlebar_border_thickness")
            .push_str(px.to_string())
            .transmute()
    }

    pub fn titlebar_padding(self) -> Command<TitlebarPadding> {
        self.push_str("titlebar_padding").transmute()
    }

    pub fn hide_edge_borders(self) -> Command<HideEdgeBorders> {
        self.push_str("hide_edge_borders").transmute()
    }

    //TODO: input

    //TODO: seat

    pub fn kill(self) -> Command<Gaps> {
        self.push_str("kill").transmute()
    }

    pub fn smart_borders(self) -> Command<SmartBoarders> {
        self.push_str("smart_borders").transmute()
    }

    pub fn smart_gaps(self) -> Command<SmartGaps> {
        self.push_str("smart_gaps").transmute()
    }

    pub fn mark(self) -> Command<Mark> {
        self.push_str("mark").transmute()
    }

    //TODO: add missing subcommands
    pub fn mode(self, mode: impl AsRef<str>) -> Command<Valid> {
        self.push_str("mode").push_str(mode).transmute()
    }

    pub fn mouse_warping(self) -> Command<MouseWraping> {
        self.push_str("mouse_warping").transmute()
    }

    pub fn no_focus(self, criteria: impl AsRef<str>) -> Command<Valid> {
        self.push_str("no_focus").push_str(criteria).transmute()
    }

    //TODO: output

    pub fn popup_during_fullscreen(self) -> Command<PopupDuringFullscreen> {
        self.push_str("popup_during_fullscreen").transmute()
    }

    pub fn set(self, key: impl AsRef<str>, value: impl AsRef<str>) -> Command<Valid> {
        self.push_str(key).push_str(value).transmute()
    }

    pub fn show_marks(self) -> Command<ShowMarks> {
        self.push_str("show_marks").transmute()
    }

    pub fn opacity(self) -> Command<Opacity> {
        self.push_str("opacity").transmute()
    }

    pub fn tiling_drag(self) -> Command<TilingDrag> {
        self.push_str("smart_borders").transmute()
    }

    pub fn tiling_drag_threshold(self, threshold: usize) -> Command<TilingDrag> {
        self.push_str("tiling_drag_threshold")
            .push_str(threshold.to_string())
            .transmute()
    }

    pub fn title_align(self) -> Command<TitleAlign> {
        self.push_str("title_align").transmute()
    }

    pub fn unbindswitch(self) -> Command<Unbindswitch> {
        self.push_str("unbindswitch").transmute()
    }

    //TODO: unbindsym

    //TODO: unbindcode

    pub fn unmark(self) -> Command<Unmark> {
        self.push_str("unmark").transmute()
    }

    pub fn urgent(self) -> Command<Urgent> {
        self.push_str("urgent").transmute()
    }

    pub fn workspace(self) -> Command<Workspace> {
        self.push_str("workspace").transmute()
    }

    pub fn workspace_auto_back_and_forth(self) -> Command<WorkspaceAutoBackAndForth> {
        self.push_str("workspace_auto_back_and_forth").transmute()
    }
}
