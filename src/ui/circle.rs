use std::slice;
use tui::style::Color;
use tui::widgets::canvas::Shape;

pub struct PointsIterator<'a> {
    iter: slice::Iter<'a, (f64, f64)>,
}

impl<'a> From<&'a [(f64, f64)]> for PointsIterator<'a> {
    fn from(data: &'a [(f64, f64)]) -> PointsIterator<'a> {
        PointsIterator { iter: data.iter() }
    }
}

impl<'a> Iterator for PointsIterator<'a> {
    type Item = (f64, f64);
    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(p) => Some(*p),
            None => None,
        }
    }
}

/// Shape to draw a world map with the given resolution and color
pub struct Circle {
    pub circle: &'static [(f64, f64)],
    pub color: Color,
}

impl Default for Circle {
    fn default() -> Circle {
        Circle {
            circle: &CIRCLE,
            color: Color::Reset,
        }
    }
}

impl<'a> Shape<'a> for Circle {
    fn color(&self) -> Color {
        self.color
    }
    fn points(&'a self) -> Box<dyn Iterator<Item = (f64, f64)> + 'a> {
        Box::new(self.into_iter())
    }
}

impl<'a> IntoIterator for &'a Circle {
    type Item = (f64, f64);
    type IntoIter = PointsIterator<'a>;
    fn into_iter(self) -> Self::IntoIter {
        PointsIterator::from(self.circle)
    }
}

pub static CIRCLE: [(f64, f64); 250] = [
    (49.9842, 1.2565),
    (49.9368, 2.5122),
    (49.8579, 3.7663),
    (49.7476, 5.0181),
    (49.6057, 6.2667),
    (49.4326, 7.5113),
    (49.2282, 8.7512),
    (48.9928, 9.9855),
    (48.7263, 11.2135),
    (48.4292, 12.4345),
    (48.1014, 13.6476),
    (47.7432, 14.8521),
    (47.3549, 16.0472),
    (46.9367, 17.2321),
    (46.4888, 18.4062),
    (46.0116, 19.5687),
    (45.5053, 20.7188),
    (44.9703, 21.8558),
    (44.4068, 22.979),
    (43.8153, 24.0877),
    (43.1962, 25.1812),
    (42.5497, 26.2587),
    (41.8764, 27.3197),
    (41.1766, 28.3634),
    (40.4508, 29.3893),
    (39.6995, 30.3965),
    (38.9231, 31.3846),
    (38.1221, 32.3528),
    (37.2971, 33.3006),
    (36.4484, 34.2274),
    (35.5768, 35.1325),
    (34.6827, 36.0155),
    (33.7666, 36.8757),
    (32.8293, 37.7126),
    (31.8712, 38.5257),
    (30.893, 39.3144),
    (29.8952, 40.0783),
    (28.8786, 40.817),
    (27.8438, 41.5298),
    (26.7913, 42.2164),
    (25.722, 42.8763),
    (24.6364, 43.5092),
    (23.5352, 44.1146),
    (22.4192, 44.6921),
    (21.289, 45.2414),
    (20.1453, 45.7621),
    (18.989, 46.2539),
    (17.8206, 46.7164),
    (16.641, 47.1495),
    (15.4508, 47.5528),
    (14.251, 47.9261),
    (13.0421, 48.2691),
    (11.8249, 48.5816),
    (10.6004, 48.8634),
    (9.3691, 49.1144),
    (8.1319, 49.3343),
    (6.8895, 49.5231),
    (5.6428, 49.6806),
    (4.3926, 49.8067),
    (3.1395, 49.9013),
    (1.8845, 49.9645),
    (0.6283, 49.9961),
    (-0.6283, 49.9961),
    (-1.8845, 49.9645),
    (-3.1395, 49.9013),
    (-4.3926, 49.8067),
    (-5.6428, 49.6806),
    (-6.8895, 49.5231),
    (-8.1319, 49.3343),
    (-9.3691, 49.1144),
    (-10.6004, 48.8634),
    (-11.8249, 48.5816),
    (-13.0421, 48.2691),
    (-14.251, 47.9261),
    (-15.4508, 47.5528),
    (-16.641, 47.1495),
    (-17.8206, 46.7164),
    (-18.989, 46.2539),
    (-20.1453, 45.7621),
    (-21.289, 45.2414),
    (-22.4192, 44.6921),
    (-23.5352, 44.1146),
    (-24.6364, 43.5092),
    (-25.722, 42.8763),
    (-26.7913, 42.2164),
    (-27.8438, 41.5298),
    (-28.8786, 40.817),
    (-29.8952, 40.0783),
    (-30.893, 39.3144),
    (-31.8712, 38.5257),
    (-32.8293, 37.7126),
    (-33.7666, 36.8757),
    (-34.6827, 36.0155),
    (-35.5768, 35.1325),
    (-36.4484, 34.2274),
    (-37.2971, 33.3006),
    (-38.1221, 32.3528),
    (-38.9231, 31.3846),
    (-39.6995, 30.3965),
    (-40.4508, 29.3893),
    (-41.1766, 28.3634),
    (-41.8764, 27.3197),
    (-42.5497, 26.2587),
    (-43.1962, 25.1812),
    (-43.8153, 24.0877),
    (-44.4068, 22.979),
    (-44.9703, 21.8558),
    (-45.5053, 20.7188),
    (-46.0116, 19.5687),
    (-46.4888, 18.4062),
    (-46.9367, 17.2321),
    (-47.3549, 16.0472),
    (-47.7432, 14.8521),
    (-48.1014, 13.6476),
    (-48.4292, 12.4345),
    (-48.7263, 11.2135),
    (-48.9928, 9.9855),
    (-49.2282, 8.7512),
    (-49.4326, 7.5113),
    (-49.6057, 6.2667),
    (-49.7476, 5.0181),
    (-49.8579, 3.7663),
    (-49.9368, 2.5122),
    (-49.9842, 1.2565),
    (-50.0, 0.0),
    (-49.9842, -1.2565),
    (-49.9368, -2.5122),
    (-49.8579, -3.7663),
    (-49.7476, -5.0181),
    (-49.6057, -6.2667),
    (-49.4326, -7.5113),
    (-49.2282, -8.7512),
    (-48.9928, -9.9855),
    (-48.7263, -11.2135),
    (-48.4292, -12.4345),
    (-48.1014, -13.6476),
    (-47.7432, -14.8521),
    (-47.3549, -16.0472),
    (-46.9367, -17.2321),
    (-46.4888, -18.4062),
    (-46.0116, -19.5687),
    (-45.5053, -20.7188),
    (-44.9703, -21.8558),
    (-44.4068, -22.979),
    (-43.8153, -24.0877),
    (-43.1962, -25.1812),
    (-42.5497, -26.2587),
    (-41.8764, -27.3197),
    (-41.1766, -28.3634),
    (-40.4508, -29.3893),
    (-39.6995, -30.3965),
    (-38.9231, -31.3846),
    (-38.1221, -32.3528),
    (-37.2971, -33.3006),
    (-36.4484, -34.2274),
    (-35.5768, -35.1325),
    (-34.6827, -36.0155),
    (-33.7666, -36.8757),
    (-32.8293, -37.7126),
    (-31.8712, -38.5257),
    (-30.893, -39.3144),
    (-29.8952, -40.0783),
    (-28.8786, -40.817),
    (-27.8438, -41.5298),
    (-26.7913, -42.2164),
    (-25.722, -42.8763),
    (-24.6364, -43.5092),
    (-23.5352, -44.1146),
    (-22.4192, -44.6921),
    (-21.289, -45.2414),
    (-20.1453, -45.7621),
    (-18.989, -46.2539),
    (-17.8206, -46.7164),
    (-16.641, -47.1495),
    (-15.4508, -47.5528),
    (-14.251, -47.9261),
    (-13.0421, -48.2691),
    (-11.8249, -48.5816),
    (-10.6004, -48.8634),
    (-9.3691, -49.1144),
    (-8.1319, -49.3343),
    (-6.8895, -49.5231),
    (-5.6428, -49.6806),
    (-4.3926, -49.8067),
    (-3.1395, -49.9013),
    (-1.8845, -49.9645),
    (-0.6283, -49.9961),
    (0.6283, -49.9961),
    (1.8845, -49.9645),
    (3.1395, -49.9013),
    (4.3926, -49.8067),
    (5.6428, -49.6806),
    (6.8895, -49.5231),
    (8.1319, -49.3343),
    (9.3691, -49.1144),
    (10.6004, -48.8634),
    (11.8249, -48.5816),
    (13.0421, -48.2691),
    (14.251, -47.9261),
    (15.4508, -47.5528),
    (16.641, -47.1495),
    (17.8206, -46.7164),
    (18.989, -46.2539),
    (20.1453, -45.7621),
    (21.289, -45.2414),
    (22.4192, -44.6921),
    (23.5352, -44.1146),
    (24.6364, -43.5092),
    (25.722, -42.8763),
    (26.7913, -42.2164),
    (27.8438, -41.5298),
    (28.8786, -40.817),
    (29.8952, -40.0783),
    (30.893, -39.3144),
    (31.8712, -38.5257),
    (32.8293, -37.7126),
    (33.7666, -36.8757),
    (34.6827, -36.0155),
    (35.5768, -35.1325),
    (36.4484, -34.2274),
    (37.2971, -33.3006),
    (38.1221, -32.3528),
    (38.9231, -31.3846),
    (39.6995, -30.3965),
    (40.4508, -29.3893),
    (41.1766, -28.3634),
    (41.8764, -27.3197),
    (42.5497, -26.2587),
    (43.1962, -25.1812),
    (43.8153, -24.0877),
    (44.4068, -22.979),
    (44.9703, -21.8558),
    (45.5053, -20.7188),
    (46.0116, -19.5687),
    (46.4888, -18.4062),
    (46.9367, -17.2321),
    (47.3549, -16.0472),
    (47.7432, -14.8521),
    (48.1014, -13.6476),
    (48.4292, -12.4345),
    (48.7263, -11.2135),
    (48.9928, -9.9855),
    (49.2282, -8.7512),
    (49.4326, -7.5113),
    (49.6057, -6.2667),
    (49.7476, -5.0181),
    (49.8579, -3.7663),
    (49.9368, -2.5122),
    (49.9842, -1.2565),
    (50.0, -0.0),
];

pub static CIRCLE_TICK: [(f64, f64); 250] = [
    (49.9961, 0.6283),
    (49.9645, 1.8845),
    (49.9013, 3.1395),
    (49.8067, 4.3926),
    (49.6806, 5.6428),
    (49.5231, 6.8895),
    (49.3343, 8.1319),
    (49.1144, 9.3691),
    (48.8634, 10.6004),
    (48.5816, 11.8249),
    (48.2691, 13.0421),
    (47.9261, 14.251),
    (47.5528, 15.4508),
    (47.1495, 16.641),
    (46.7164, 17.8206),
    (46.2539, 18.989),
    (45.7621, 20.1453),
    (45.2414, 21.289),
    (44.6921, 22.4192),
    (44.1146, 23.5352),
    (43.5092, 24.6364),
    (42.8763, 25.722),
    (42.2164, 26.7913),
    (41.5298, 27.8438),
    (40.817, 28.8786),
    (40.0783, 29.8952),
    (39.3144, 30.893),
    (38.5257, 31.8712),
    (37.7126, 32.8293),
    (36.8757, 33.7666),
    (36.0155, 34.6827),
    (35.1325, 35.5768),
    (34.2274, 36.4484),
    (33.3006, 37.2971),
    (32.3528, 38.1221),
    (31.3846, 38.9231),
    (30.3965, 39.6995),
    (29.3893, 40.4508),
    (28.3634, 41.1766),
    (27.3197, 41.8764),
    (26.2587, 42.5497),
    (25.1812, 43.1962),
    (24.0877, 43.8153),
    (22.979, 44.4068),
    (21.8558, 44.9703),
    (20.7188, 45.5053),
    (19.5687, 46.0116),
    (18.4062, 46.4888),
    (17.2321, 46.9367),
    (16.0472, 47.3549),
    (14.8521, 47.7432),
    (13.6476, 48.1014),
    (12.4345, 48.4292),
    (11.2135, 48.7263),
    (9.9855, 48.9928),
    (8.7512, 49.2282),
    (7.5113, 49.4326),
    (6.2667, 49.6057),
    (5.0181, 49.7476),
    (3.7663, 49.8579),
    (2.5122, 49.9368),
    (1.2565, 49.9842),
    (0.0, 50.0),
    (-1.2565, 49.9842),
    (-2.5122, 49.9368),
    (-3.7663, 49.8579),
    (-5.0181, 49.7476),
    (-6.2667, 49.6057),
    (-7.5113, 49.4326),
    (-8.7512, 49.2282),
    (-9.9855, 48.9928),
    (-11.2135, 48.7263),
    (-12.4345, 48.4292),
    (-13.6476, 48.1014),
    (-14.8521, 47.7432),
    (-16.0472, 47.3549),
    (-17.2321, 46.9367),
    (-18.4062, 46.4888),
    (-19.5687, 46.0116),
    (-20.7188, 45.5053),
    (-21.8558, 44.9703),
    (-22.979, 44.4068),
    (-24.0877, 43.8153),
    (-25.1812, 43.1962),
    (-26.2587, 42.5497),
    (-27.3197, 41.8764),
    (-28.3634, 41.1766),
    (-29.3893, 40.4508),
    (-30.3965, 39.6995),
    (-31.3846, 38.9231),
    (-32.3528, 38.1221),
    (-33.3006, 37.2971),
    (-34.2274, 36.4484),
    (-35.1325, 35.5768),
    (-36.0155, 34.6827),
    (-36.8757, 33.7666),
    (-37.7126, 32.8293),
    (-38.5257, 31.8712),
    (-39.3144, 30.893),
    (-40.0783, 29.8952),
    (-40.817, 28.8786),
    (-41.5298, 27.8438),
    (-42.2164, 26.7913),
    (-42.8763, 25.722),
    (-43.5092, 24.6364),
    (-44.1146, 23.5352),
    (-44.6921, 22.4192),
    (-45.2414, 21.289),
    (-45.7621, 20.1453),
    (-46.2539, 18.989),
    (-46.7164, 17.8206),
    (-47.1495, 16.641),
    (-47.5528, 15.4508),
    (-47.9261, 14.251),
    (-48.2691, 13.0421),
    (-48.5816, 11.8249),
    (-48.8634, 10.6004),
    (-49.1144, 9.3691),
    (-49.3343, 8.1319),
    (-49.5231, 6.8895),
    (-49.6806, 5.6428),
    (-49.8067, 4.3926),
    (-49.9013, 3.1395),
    (-49.9645, 1.8845),
    (-49.9961, 0.6283),
    (-49.9961, -0.6283),
    (-49.9645, -1.8845),
    (-49.9013, -3.1395),
    (-49.8067, -4.3926),
    (-49.6806, -5.6428),
    (-49.5231, -6.8895),
    (-49.3343, -8.1319),
    (-49.1144, -9.3691),
    (-48.8634, -10.6004),
    (-48.5816, -11.8249),
    (-48.2691, -13.0421),
    (-47.9261, -14.251),
    (-47.5528, -15.4508),
    (-47.1495, -16.641),
    (-46.7164, -17.8206),
    (-46.2539, -18.989),
    (-45.7621, -20.1453),
    (-45.2414, -21.289),
    (-44.6921, -22.4192),
    (-44.1146, -23.5352),
    (-43.5092, -24.6364),
    (-42.8763, -25.722),
    (-42.2164, -26.7913),
    (-41.5298, -27.8438),
    (-40.817, -28.8786),
    (-40.0783, -29.8952),
    (-39.3144, -30.893),
    (-38.5257, -31.8712),
    (-37.7126, -32.8293),
    (-36.8757, -33.7666),
    (-36.0155, -34.6827),
    (-35.1325, -35.5768),
    (-34.2274, -36.4484),
    (-33.3006, -37.2971),
    (-32.3528, -38.1221),
    (-31.3846, -38.9231),
    (-30.3965, -39.6995),
    (-29.3893, -40.4508),
    (-28.3634, -41.1766),
    (-27.3197, -41.8764),
    (-26.2587, -42.5497),
    (-25.1812, -43.1962),
    (-24.0877, -43.8153),
    (-22.979, -44.4068),
    (-21.8558, -44.9703),
    (-20.7188, -45.5053),
    (-19.5687, -46.0116),
    (-18.4062, -46.4888),
    (-17.2321, -46.9367),
    (-16.0472, -47.3549),
    (-14.8521, -47.7432),
    (-13.6476, -48.1014),
    (-12.4345, -48.4292),
    (-11.2135, -48.7263),
    (-9.9855, -48.9928),
    (-8.7512, -49.2282),
    (-7.5113, -49.4326),
    (-6.2667, -49.6057),
    (-5.0181, -49.7476),
    (-3.7663, -49.8579),
    (-2.5122, -49.9368),
    (-1.2565, -49.9842),
    (-0.0, -50.0),
    (1.2565, -49.9842),
    (2.5122, -49.9368),
    (3.7663, -49.8579),
    (5.0181, -49.7476),
    (6.2667, -49.6057),
    (7.5113, -49.4326),
    (8.7512, -49.2282),
    (9.9855, -48.9928),
    (11.2135, -48.7263),
    (12.4345, -48.4292),
    (13.6476, -48.1014),
    (14.8521, -47.7432),
    (16.0472, -47.3549),
    (17.2321, -46.9367),
    (18.4062, -46.4888),
    (19.5687, -46.0116),
    (20.7188, -45.5053),
    (21.8558, -44.9703),
    (22.979, -44.4068),
    (24.0877, -43.8153),
    (25.1812, -43.1962),
    (26.2587, -42.5497),
    (27.3197, -41.8764),
    (28.3634, -41.1766),
    (29.3893, -40.4508),
    (30.3965, -39.6995),
    (31.3846, -38.9231),
    (32.3528, -38.1221),
    (33.3006, -37.2971),
    (34.2274, -36.4484),
    (35.1325, -35.5768),
    (36.0155, -34.6827),
    (36.8757, -33.7666),
    (37.7126, -32.8293),
    (38.5257, -31.8712),
    (39.3144, -30.893),
    (40.0783, -29.8952),
    (40.817, -28.8786),
    (41.5298, -27.8438),
    (42.2164, -26.7913),
    (42.8763, -25.722),
    (43.5092, -24.6364),
    (44.1146, -23.5352),
    (44.6921, -22.4192),
    (45.2414, -21.289),
    (45.7621, -20.1453),
    (46.2539, -18.989),
    (46.7164, -17.8206),
    (47.1495, -16.641),
    (47.5528, -15.4508),
    (47.9261, -14.251),
    (48.2691, -13.0421),
    (48.5816, -11.8249),
    (48.8634, -10.6004),
    (49.1144, -9.3691),
    (49.3343, -8.1319),
    (49.5231, -6.8895),
    (49.6806, -5.6428),
    (49.8067, -4.3926),
    (49.9013, -3.1395),
    (49.9645, -1.8845),
    (49.9961, -0.6283),
];