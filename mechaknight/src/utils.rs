pub fn rect_ratatui_to_bevy(rect: ratatui::layout::Rect) -> bevy::math::URect {
    bevy::math::URect::new(
        rect.left() as u32,
        rect.top() as u32,
        rect.right() as u32 - 1,
        rect.bottom() as u32 - 1,
    )
}

pub mod directions {
    use bevy::math::IVec2;

    const fn add(a: IVec2, b: IVec2) -> IVec2 {
        IVec2 {
            x: a.x + b.x,
            y: a.y + b.y,
        }
    }

    pub static UP:    IVec2 = IVec2 { x: 0,  y: -1 };
    pub static DOWN:  IVec2 = IVec2 { x: 0,  y: 1  };
    pub static LEFT:  IVec2 = IVec2 { x: -1, y: 0  };
    pub static RIGHT: IVec2 = IVec2 { x: 1,  y: 0  };

    pub static N: IVec2 = UP;
    pub static E: IVec2 = RIGHT;
    pub static S: IVec2 = DOWN;
    pub static W: IVec2 = LEFT;

    pub static NE: IVec2 = add(N, E);
    pub static SE: IVec2 = add(S, E);
    pub static SW: IVec2 = add(S, W);
    pub static NW: IVec2 = add(N, W);
}
