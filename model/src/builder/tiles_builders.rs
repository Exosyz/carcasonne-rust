use crate::side::SideKind;
use crate::tile::{TileBuilder, TileExtension};

pub trait AbbeyTileBuilder {
    fn build_a_abbey(&mut self) -> &mut Self;
    fn build_b_abbey(&mut self) -> &mut Self;
}

impl AbbeyTileBuilder for TileBuilder {
    fn build_a_abbey(&mut self) -> &mut Self {
        self.north(|s| s.kind(SideKind::Road).section(1))
            .west(|s| s.kind(SideKind::Meadow))
            .south(|s| s.kind(SideKind::Meadow))
            .east(|s| s.kind(SideKind::Meadow))
            .tile_extension(TileExtension::Abbey)
    }
    fn build_b_abbey(&mut self) -> &mut Self {
        self.north(|s| s.kind(SideKind::Meadow))
            .west(|s| s.kind(SideKind::Meadow))
            .south(|s| s.kind(SideKind::Meadow))
            .east(|s| s.kind(SideKind::Meadow))
            .tile_extension(TileExtension::Abbey)
    }
}

pub trait RoadTileBuilder {
    fn build_u_road(&mut self) -> &mut Self;
    fn build_v_road(&mut self) -> &mut Self;
    fn build_w_road(&mut self) -> &mut Self;
    fn build_x_road(&mut self) -> &mut Self;
}

impl RoadTileBuilder for TileBuilder {
    fn build_u_road(&mut self) -> &mut Self {
        self.north(|s| s.kind(SideKind::Road).section(1))
            .west(|s| s.kind(SideKind::Meadow))
            .south(|s| s.kind(SideKind::Road).section(1))
            .east(|s| s.kind(SideKind::Meadow))
    }
    fn build_v_road(&mut self) -> &mut Self {
        self.north(|s| s.kind(SideKind::Road).section(1))
            .west(|s| s.kind(SideKind::Road).section(1))
            .south(|s| s.kind(SideKind::Meadow))
            .east(|s| s.kind(SideKind::Meadow))
    }
    fn build_w_road(&mut self) -> &mut Self {
        self.north(|s| s.kind(SideKind::Road).section(1))
            .west(|s| s.kind(SideKind::Road).section(2))
            .south(|s| s.kind(SideKind::Road).section(3))
            .east(|s| s.kind(SideKind::Meadow))
    }
    fn build_x_road(&mut self) -> &mut Self {
        self.north(|s| s.kind(SideKind::Road).section(1))
            .west(|s| s.kind(SideKind::Road).section(2))
            .south(|s| s.kind(SideKind::Road).section(3))
            .east(|s| s.kind(SideKind::Road).section(4))
    }
}

pub trait TownTileBuilder {
    fn build_c_town(&mut self) -> &mut Self;
    fn build_d_town(&mut self) -> &mut Self;
    fn build_e_town(&mut self) -> &mut Self;
    fn build_f_town(&mut self) -> &mut Self;
    fn build_g_town(&mut self) -> &mut Self;
    fn build_h_town(&mut self) -> &mut Self;
    fn build_i_town(&mut self) -> &mut Self;
    fn build_j_town(&mut self) -> &mut Self;
    fn build_k_town(&mut self) -> &mut Self;
    fn build_l_town(&mut self) -> &mut Self;
    fn build_m_town(&mut self) -> &mut Self;
    fn build_n_town(&mut self) -> &mut Self;
    fn build_o_town(&mut self) -> &mut Self;
    fn build_p_town(&mut self) -> &mut Self;
    fn build_q_town(&mut self) -> &mut Self;
    fn build_r_town(&mut self) -> &mut Self;
    fn build_s_town(&mut self) -> &mut Self;
    fn build_t_town(&mut self) -> &mut Self;
}

impl TownTileBuilder for TileBuilder {
    fn build_c_town(&mut self) -> &mut Self {
        self.north(|s| s.kind(SideKind::Town).section(1))
            .west(|s| s.kind(SideKind::Town).section(1))
            .south(|s| s.kind(SideKind::Town).section(1))
            .east(|s| s.kind(SideKind::Town).section(1))
            .tile_extension(TileExtension::TownShield(1))
    }
    fn build_d_town(&mut self) -> &mut Self {
        self.north(|s| s.kind(SideKind::Town).section(1))
            .west(|s| s.kind(SideKind::Road).section(1))
            .south(|s| s.kind(SideKind::Meadow))
            .east(|s| s.kind(SideKind::Road).section(1))
    }
    fn build_e_town(&mut self) -> &mut Self {
        self.north(|s| s.kind(SideKind::Town).section(1))
            .west(|s| s.kind(SideKind::Meadow))
            .south(|s| s.kind(SideKind::Meadow))
            .east(|s| s.kind(SideKind::Meadow))
    }
    fn build_f_town(&mut self) -> &mut Self {
        self.build_g_town()
            .tile_extension(TileExtension::TownShield(1))
    }

    fn build_g_town(&mut self) -> &mut Self {
        self.north(|s| s.kind(SideKind::Meadow))
            .west(|s| s.kind(SideKind::Town).section(1))
            .south(|s| s.kind(SideKind::Meadow))
            .east(|s| s.kind(SideKind::Town).section(1))
    }

    fn build_h_town(&mut self) -> &mut Self {
        self.north(|s| s.kind(SideKind::Meadow))
            .west(|s| s.kind(SideKind::Town).section(1))
            .south(|s| s.kind(SideKind::Meadow))
            .east(|s| s.kind(SideKind::Town).section(2))
    }

    fn build_i_town(&mut self) -> &mut Self {
        self.north(|s| s.kind(SideKind::Town).section(1))
            .west(|s| s.kind(SideKind::Town).section(2))
            .south(|s| s.kind(SideKind::Meadow))
            .east(|s| s.kind(SideKind::Meadow))
    }

    fn build_j_town(&mut self) -> &mut Self {
        self.north(|s| s.kind(SideKind::Town).section(1))
            .west(|s| s.kind(SideKind::Meadow))
            .south(|s| s.kind(SideKind::Road).section(1))
            .east(|s| s.kind(SideKind::Road).section(1))
    }

    fn build_k_town(&mut self) -> &mut Self {
        self.north(|s| s.kind(SideKind::Town).section(1))
            .west(|s| s.kind(SideKind::Road).section(1))
            .south(|s| s.kind(SideKind::Meadow))
            .east(|s| s.kind(SideKind::Road).section(1))
    }

    fn build_l_town(&mut self) -> &mut Self {
        self.north(|s| s.kind(SideKind::Town).section(1))
            .west(|s| s.kind(SideKind::Road).section(1))
            .south(|s| s.kind(SideKind::Road).section(2))
            .east(|s| s.kind(SideKind::Road).section(3))
    }

    fn build_m_town(&mut self) -> &mut Self {
        self.build_n_town()
            .tile_extension(TileExtension::TownShield(1))
    }

    fn build_n_town(&mut self) -> &mut Self {
        self.north(|s| s.kind(SideKind::Town).section(1))
            .west(|s| s.kind(SideKind::Town).section(1))
            .south(|s| s.kind(SideKind::Meadow))
            .east(|s| s.kind(SideKind::Meadow))
    }

    fn build_o_town(&mut self) -> &mut Self {
        self.build_p_town()
            .tile_extension(TileExtension::TownShield(1))
    }

    fn build_p_town(&mut self) -> &mut Self {
        self.north(|s| s.kind(SideKind::Town).section(1))
            .west(|s| s.kind(SideKind::Town).section(1))
            .south(|s| s.kind(SideKind::Road).section(1))
            .east(|s| s.kind(SideKind::Road).section(1))
    }

    fn build_q_town(&mut self) -> &mut Self {
        self.build_r_town()
            .tile_extension(TileExtension::TownShield(1))
    }

    fn build_r_town(&mut self) -> &mut Self {
        self.north(|s| s.kind(SideKind::Town).section(1))
            .west(|s| s.kind(SideKind::Town).section(1))
            .south(|s| s.kind(SideKind::Meadow))
            .east(|s| s.kind(SideKind::Town).section(1))
    }

    fn build_s_town(&mut self) -> &mut Self {
        self.build_t_town()
            .tile_extension(TileExtension::TownShield(1))
    }

    fn build_t_town(&mut self) -> &mut Self {
        self.north(|s| s.kind(SideKind::Town).section(1))
            .west(|s| s.kind(SideKind::Town).section(1))
            .south(|s| s.kind(SideKind::Road).section(1))
            .east(|s| s.kind(SideKind::Town).section(1))
    }
}
