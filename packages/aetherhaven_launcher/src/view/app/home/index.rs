use freya::prelude::*;

use crate::components::{ActiveClusterPanel, RecentsRow};
use crate::theme;
use crate::theme::colors;

#[derive(PartialEq)]
pub struct Home;

impl Component for Home {
    fn render(&self) -> impl IntoElement {
        rect()
            .vertical()
            .width(Size::fill())
            .height(Size::fill())
            .content(Content::Flex)
            .overflow(Overflow::Clip)
            .padding(Gaps::new(28., theme::HOME_PADDING_PX, 24., theme::HOME_PADDING_PX))
            .spacing(16.)
            .child(
                rect()
                    .horizontal()
                    .width(Size::fill())
                    .cross_align(Alignment::Center)
                    .child(
                        label()
                            .text("AETHERHAVEN / HOME")
                            .font_size(12.)
                            .font_weight(FontWeight::BOLD)
                            .color(colors::brand()),
                    )
                    .child(
                        rect()
                            .width(Size::flex(1.))
                            .height(Size::px(1.))
                            .margin(Gaps::new_symmetric(0., 14.))
                            .background(colors::component_border()),
                    ),
            )
            .child(
                rect()
                    .horizontal()
                    .width(Size::fill())
                    .height(Size::flex(1.0))
                    .min_height(Size::px(300.))
                    .spacing(16.)
                    .cross_align(Alignment::Center)
                    .child(
                        rect()
                            .width(Size::flex(1.0))
                            .height(Size::fill())
                            .padding(Gaps::new(28., 30., 28., 30.))
                            .main_align(Alignment::Center)
                            .child(ActiveClusterPanel),
                    ),
            )
            .child(
                rect()
                    .horizontal()
                    .width(Size::fill())
                    .cross_align(Alignment::Center)
                    .spacing(12.)
                    .child(
                        label()
                            .text("YOUR WORLDS")
                            .font_size(12.)
                            .font_weight(FontWeight::SEMI_BOLD)
                            .color(colors::brand()),
                    )
                    .child(
                        rect()
                            .width(Size::flex(1.))
                            .height(Size::px(1.))
                            .background(colors::component_border()),
                    ),
            )
            .child(RecentsRow)
    }
}
