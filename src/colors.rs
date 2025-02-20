use image::Rgb;
use kd_tree::KdTree3;
use std::sync::OnceLock;
use typenum::U3;

/// The number of colors that Minecraft supports, excluding the 4 transparent ones.
const COLOR_COUNT: usize = 244;

/// A wrapper around `Rgb<u8>` that implements the `KdPoint` trait for use with the `kd-tree`.
/// Contains the original `Rgb<u8>` value and the index of that color in the Minecraft color list.
#[derive(Debug, Clone, Copy)]
pub struct SearchableRgb(Rgb<u8>);
impl kd_tree::KdPoint for SearchableRgb {
    type Scalar = isize;
    type Dim = U3;

    fn at(&self, k: usize) -> Self::Scalar {
        self.0[k] as isize
    }
}

/// This array contains `COLOR_COUNT` entries
/// Each element in this array represents a Minecraft map color
/// As of Minecraft 1.21, there are 244 colours in this array.
#[allow(overflowing_literals)]
const COLOR_LIST: [SearchableRgb; COLOR_COUNT] = [
    SearchableRgb(Rgb([89, 125, 39])),
    SearchableRgb(Rgb([109, 153, 48])),
    SearchableRgb(Rgb([127, 178, 56])),
    SearchableRgb(Rgb([67, 94, 29])),
    SearchableRgb(Rgb([174, 164, 115])),
    SearchableRgb(Rgb([213, 201, 140])),
    SearchableRgb(Rgb([247, 233, 163])),
    SearchableRgb(Rgb([130, 123, 86])),
    SearchableRgb(Rgb([140, 140, 140])),
    SearchableRgb(Rgb([171, 171, 171])),
    SearchableRgb(Rgb([199, 199, 199])),
    SearchableRgb(Rgb([105, 105, 105])),
    SearchableRgb(Rgb([180, 0, 0])),
    SearchableRgb(Rgb([220, 0, 0])),
    SearchableRgb(Rgb([255, 0, 0])),
    SearchableRgb(Rgb([135, 0, 0])),
    SearchableRgb(Rgb([112, 112, 180])),
    SearchableRgb(Rgb([138, 138, 220])),
    SearchableRgb(Rgb([160, 160, 255])),
    SearchableRgb(Rgb([84, 84, 135])),
    SearchableRgb(Rgb([117, 117, 117])),
    SearchableRgb(Rgb([144, 144, 144])),
    SearchableRgb(Rgb([167, 167, 167])),
    SearchableRgb(Rgb([88, 88, 88])),
    SearchableRgb(Rgb([0, 87, 0])),
    SearchableRgb(Rgb([0, 106, 0])),
    SearchableRgb(Rgb([0, 124, 0])),
    SearchableRgb(Rgb([0, 65, 0])),
    SearchableRgb(Rgb([180, 180, 180])),
    SearchableRgb(Rgb([220, 220, 220])),
    SearchableRgb(Rgb([255, 255, 255])),
    SearchableRgb(Rgb([135, 135, 135])),
    SearchableRgb(Rgb([115, 118, 129])),
    SearchableRgb(Rgb([141, 144, 158])),
    SearchableRgb(Rgb([164, 168, 184])),
    SearchableRgb(Rgb([86, 88, 97])),
    SearchableRgb(Rgb([106, 76, 54])),
    SearchableRgb(Rgb([130, 94, 66])),
    SearchableRgb(Rgb([151, 109, 77])),
    SearchableRgb(Rgb([79, 57, 40])),
    SearchableRgb(Rgb([79, 79, 79])),
    SearchableRgb(Rgb([96, 96, 96])),
    SearchableRgb(Rgb([112, 112, 112])),
    SearchableRgb(Rgb([59, 59, 59])),
    SearchableRgb(Rgb([45, 45, 180])),
    SearchableRgb(Rgb([55, 55, 220])),
    SearchableRgb(Rgb([64, 64, 255])),
    SearchableRgb(Rgb([33, 33, 135])),
    SearchableRgb(Rgb([100, 84, 50])),
    SearchableRgb(Rgb([123, 102, 62])),
    SearchableRgb(Rgb([143, 119, 72])),
    SearchableRgb(Rgb([75, 63, 38])),
    SearchableRgb(Rgb([180, 177, 172])),
    SearchableRgb(Rgb([220, 217, 211])),
    SearchableRgb(Rgb([255, 252, 245])),
    SearchableRgb(Rgb([135, 133, 129])),
    SearchableRgb(Rgb([152, 89, 36])),
    SearchableRgb(Rgb([186, 109, 44])),
    SearchableRgb(Rgb([216, 127, 51])),
    SearchableRgb(Rgb([114, 67, 27])),
    SearchableRgb(Rgb([125, 53, 152])),
    SearchableRgb(Rgb([153, 65, 186])),
    SearchableRgb(Rgb([178, 76, 216])),
    SearchableRgb(Rgb([94, 40, 114])),
    SearchableRgb(Rgb([72, 108, 152])),
    SearchableRgb(Rgb([88, 132, 186])),
    SearchableRgb(Rgb([102, 153, 216])),
    SearchableRgb(Rgb([54, 81, 114])),
    SearchableRgb(Rgb([161, 161, 36])),
    SearchableRgb(Rgb([197, 197, 44])),
    SearchableRgb(Rgb([229, 229, 51])),
    SearchableRgb(Rgb([121, 121, 27])),
    SearchableRgb(Rgb([89, 144, 17])),
    SearchableRgb(Rgb([109, 176, 21])),
    SearchableRgb(Rgb([127, 204, 25])),
    SearchableRgb(Rgb([67, 108, 13])),
    SearchableRgb(Rgb([170, 89, 116])),
    SearchableRgb(Rgb([208, 109, 142])),
    SearchableRgb(Rgb([242, 127, 165])),
    SearchableRgb(Rgb([128, 67, 87])),
    SearchableRgb(Rgb([53, 53, 53])),
    SearchableRgb(Rgb([65, 65, 65])),
    SearchableRgb(Rgb([76, 76, 76])),
    SearchableRgb(Rgb([40, 40, 40])),
    SearchableRgb(Rgb([108, 108, 108])),
    SearchableRgb(Rgb([132, 132, 132])),
    SearchableRgb(Rgb([153, 153, 153])),
    SearchableRgb(Rgb([81, 81, 81])),
    SearchableRgb(Rgb([53, 89, 108])),
    SearchableRgb(Rgb([65, 109, 132])),
    SearchableRgb(Rgb([76, 127, 153])),
    SearchableRgb(Rgb([40, 67, 81])),
    SearchableRgb(Rgb([89, 44, 125])),
    SearchableRgb(Rgb([109, 54, 153])),
    SearchableRgb(Rgb([127, 63, 178])),
    SearchableRgb(Rgb([67, 33, 94])),
    SearchableRgb(Rgb([36, 53, 125])),
    SearchableRgb(Rgb([44, 65, 153])),
    SearchableRgb(Rgb([51, 76, 178])),
    SearchableRgb(Rgb([27, 40, 94])),
    SearchableRgb(Rgb([72, 53, 36])),
    SearchableRgb(Rgb([88, 65, 44])),
    SearchableRgb(Rgb([102, 76, 51])),
    SearchableRgb(Rgb([54, 40, 27])),
    SearchableRgb(Rgb([72, 89, 36])),
    SearchableRgb(Rgb([88, 109, 44])),
    SearchableRgb(Rgb([102, 127, 51])),
    SearchableRgb(Rgb([54, 67, 27])),
    SearchableRgb(Rgb([108, 36, 36])),
    SearchableRgb(Rgb([132, 44, 44])),
    SearchableRgb(Rgb([153, 51, 51])),
    SearchableRgb(Rgb([81, 27, 27])),
    SearchableRgb(Rgb([17, 17, 17])),
    SearchableRgb(Rgb([21, 21, 21])),
    SearchableRgb(Rgb([25, 25, 25])),
    SearchableRgb(Rgb([13, 13, 13])),
    SearchableRgb(Rgb([176, 168, 54])),
    SearchableRgb(Rgb([215, 205, 66])),
    SearchableRgb(Rgb([250, 238, 77])),
    SearchableRgb(Rgb([132, 126, 40])),
    SearchableRgb(Rgb([64, 154, 150])),
    SearchableRgb(Rgb([79, 188, 183])),
    SearchableRgb(Rgb([92, 219, 213])),
    SearchableRgb(Rgb([48, 115, 112])),
    SearchableRgb(Rgb([52, 90, 180])),
    SearchableRgb(Rgb([63, 110, 220])),
    SearchableRgb(Rgb([74, 128, 255])),
    SearchableRgb(Rgb([39, 67, 135])),
    SearchableRgb(Rgb([0, 153, 40])),
    SearchableRgb(Rgb([0, 187, 50])),
    SearchableRgb(Rgb([0, 217, 58])),
    SearchableRgb(Rgb([0, 114, 30])),
    SearchableRgb(Rgb([91, 60, 34])),
    SearchableRgb(Rgb([111, 74, 42])),
    SearchableRgb(Rgb([129, 86, 49])),
    SearchableRgb(Rgb([68, 45, 25])),
    SearchableRgb(Rgb([79, 1, 0])),
    SearchableRgb(Rgb([96, 1, 0])),
    SearchableRgb(Rgb([112, 2, 0])),
    SearchableRgb(Rgb([59, 1, 0])),
    SearchableRgb(Rgb([147, 124, 113])),
    SearchableRgb(Rgb([180, 152, 138])),
    SearchableRgb(Rgb([209, 177, 161])),
    SearchableRgb(Rgb([110, 93, 85])),
    SearchableRgb(Rgb([112, 57, 25])),
    SearchableRgb(Rgb([137, 70, 31])),
    SearchableRgb(Rgb([159, 82, 36])),
    SearchableRgb(Rgb([84, 43, 19])),
    SearchableRgb(Rgb([105, 61, 76])),
    SearchableRgb(Rgb([128, 75, 93])),
    SearchableRgb(Rgb([149, 87, 108])),
    SearchableRgb(Rgb([78, 46, 57])),
    SearchableRgb(Rgb([79, 76, 97])),
    SearchableRgb(Rgb([96, 93, 119])),
    SearchableRgb(Rgb([112, 108, 138])),
    SearchableRgb(Rgb([59, 57, 73])),
    SearchableRgb(Rgb([131, 93, 25])),
    SearchableRgb(Rgb([160, 114, 31])),
    SearchableRgb(Rgb([186, 133, 36])),
    SearchableRgb(Rgb([98, 70, 19])),
    SearchableRgb(Rgb([72, 82, 37])),
    SearchableRgb(Rgb([88, 100, 45])),
    SearchableRgb(Rgb([103, 117, 53])),
    SearchableRgb(Rgb([54, 61, 28])),
    SearchableRgb(Rgb([112, 54, 55])),
    SearchableRgb(Rgb([138, 66, 67])),
    SearchableRgb(Rgb([160, 77, 78])),
    SearchableRgb(Rgb([84, 40, 41])),
    SearchableRgb(Rgb([40, 28, 24])),
    SearchableRgb(Rgb([49, 35, 30])),
    SearchableRgb(Rgb([57, 41, 35])),
    SearchableRgb(Rgb([30, 21, 18])),
    SearchableRgb(Rgb([95, 75, 69])),
    SearchableRgb(Rgb([116, 92, 84])),
    SearchableRgb(Rgb([135, 107, 98])),
    SearchableRgb(Rgb([71, 56, 51])),
    SearchableRgb(Rgb([61, 64, 64])),
    SearchableRgb(Rgb([75, 79, 79])),
    SearchableRgb(Rgb([87, 92, 92])),
    SearchableRgb(Rgb([46, 48, 48])),
    SearchableRgb(Rgb([86, 51, 62])),
    SearchableRgb(Rgb([105, 62, 75])),
    SearchableRgb(Rgb([122, 73, 88])),
    SearchableRgb(Rgb([64, 38, 46])),
    SearchableRgb(Rgb([53, 43, 64])),
    SearchableRgb(Rgb([65, 53, 79])),
    SearchableRgb(Rgb([76, 62, 92])),
    SearchableRgb(Rgb([40, 32, 48])),
    SearchableRgb(Rgb([53, 35, 24])),
    SearchableRgb(Rgb([65, 43, 30])),
    SearchableRgb(Rgb([76, 50, 35])),
    SearchableRgb(Rgb([40, 26, 18])),
    SearchableRgb(Rgb([53, 57, 29])),
    SearchableRgb(Rgb([65, 70, 36])),
    SearchableRgb(Rgb([76, 82, 42])),
    SearchableRgb(Rgb([40, 43, 22])),
    SearchableRgb(Rgb([100, 42, 32])),
    SearchableRgb(Rgb([122, 51, 39])),
    SearchableRgb(Rgb([142, 60, 46])),
    SearchableRgb(Rgb([75, 31, 24])),
    SearchableRgb(Rgb([26, 15, 11])),
    SearchableRgb(Rgb([31, 18, 13])),
    SearchableRgb(Rgb([37, 22, 16])),
    SearchableRgb(Rgb([19, 11, 8])),
    SearchableRgb(Rgb([133, 33, 34])),
    SearchableRgb(Rgb([163, 41, 42])),
    SearchableRgb(Rgb([189, 48, 49])),
    SearchableRgb(Rgb([100, 25, 25])),
    SearchableRgb(Rgb([104, 44, 68])),
    SearchableRgb(Rgb([127, 54, 83])),
    SearchableRgb(Rgb([148, 63, 97])),
    SearchableRgb(Rgb([78, 33, 51])),
    SearchableRgb(Rgb([64, 17, 20])),
    SearchableRgb(Rgb([79, 21, 25])),
    SearchableRgb(Rgb([92, 25, 29])),
    SearchableRgb(Rgb([48, 13, 15])),
    SearchableRgb(Rgb([15, 88, 94])),
    SearchableRgb(Rgb([18, 108, 115])),
    SearchableRgb(Rgb([22, 126, 134])),
    SearchableRgb(Rgb([11, 66, 70])),
    SearchableRgb(Rgb([40, 100, 98])),
    SearchableRgb(Rgb([50, 122, 120])),
    SearchableRgb(Rgb([58, 142, 140])),
    SearchableRgb(Rgb([30, 75, 74])),
    SearchableRgb(Rgb([60, 31, 43])),
    SearchableRgb(Rgb([74, 37, 53])),
    SearchableRgb(Rgb([86, 44, 62])),
    SearchableRgb(Rgb([45, 23, 32])),
    SearchableRgb(Rgb([14, 127, 93])),
    SearchableRgb(Rgb([17, 155, 114])),
    SearchableRgb(Rgb([20, 180, 133])),
    SearchableRgb(Rgb([10, 95, 70])),
    SearchableRgb(Rgb([70, 70, 70])),
    SearchableRgb(Rgb([86, 86, 86])),
    SearchableRgb(Rgb([100, 100, 100])),
    SearchableRgb(Rgb([52, 52, 52])),
    SearchableRgb(Rgb([152, 123, 103])),
    SearchableRgb(Rgb([186, 150, 126])),
    SearchableRgb(Rgb([216, 175, 147])),
    SearchableRgb(Rgb([114, 92, 77])),
    SearchableRgb(Rgb([89, 117, 105])),
    SearchableRgb(Rgb([109, 144, 129])),
    SearchableRgb(Rgb([127, 167, 150])),
    SearchableRgb(Rgb([67, 88, 79])),
];

/// Wrapper around KdTree3
pub struct ColorTree(KdTree3<SearchableRgb>);

impl ColorTree {
    /// Returns the closest color in the Minecraft color palette and the distance to it.
    pub fn find_closest(&self, color: &Rgb<u8>) -> (Rgb<u8>, [i16; 3]) {
        // Cast to MinecraftRgb to use the KdTree, the index is ignored
        let to_search = SearchableRgb(*color);
        let nearest = self.0.nearest(&to_search).unwrap();

        // KdTree returns the squared distance in `nearest`, but we want the absolute distance
        let distance = [
            color.0[0] as i16 - nearest.item.0[0] as i16,
            color.0[1] as i16 - nearest.item.0[1] as i16,
            color.0[2] as i16 - nearest.item.0[2] as i16,
        ];

        // Return the MC index of the color and the distance
        (nearest.item.0, distance)
    }
}

static COLOR_TREE: OnceLock<ColorTree> = OnceLock::new();

pub fn get_color_tree() -> &'static ColorTree {
    COLOR_TREE.get_or_init(|| ColorTree(KdTree3::build(Vec::from(COLOR_LIST))))
}
