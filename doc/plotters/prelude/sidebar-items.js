initSidebarItems({"enum":[["FontFamily","Describes font family. This can be either a specific font family name, such as \"arial\", or a general font family class, such as \"serif\" and \"sans-serif\""],["FontStyle","Describes the font style. Such as Italic, Oblique, etc."],["FontTransform","Specifying text transformations"],["LabelAreaPosition","The enum used to specify the position of label area. This is used when we configure the label area size with the API `set_label_area_size`"],["SeriesLabelPosition","Describes where we want to put the series label"]],"struct":[["AreaSeries","An area series is similar to a line series but use a filled polygon"],["Category","The category coordinate"],["ChartBuilder","The helper object to create a chart context, which is used for the high-level figure drawing. With the help of this object, we can convert a basic drawing area into a chart context, which allows the high-level charting API being used on the drawing area."],["ChartContext","The context of the chart. This is the core object of Plotters. Any plot/chart is abstracted as this type, and any data series can be placed to the chart context."],["Circle","A circle element"],["Cross","Describe a cross"],["DynElement","The container for a dynamically dispatched element"],["EmptyElement","An empty composable element, which is the start point of an ad-hoc composable element"],["FontDesc","Describes a font"],["GroupBy","The ranged value spec that needs to be grouped. This is useful, for example, when we have an X axis is a integer and denotes days. And we are expecting the tick mark denotes weeks, in this way we can make the range spec grouping by 7 elements."],["HSLColor","The color described by HSL color space"],["LineSeries","The line series object, which takes an iterator of points in guest coordinate system and creates the element rendering the line plot"],["LogCoord","A log scaled coordinate axis"],["LogRange","The decorator type for a range of a log-scaled value"],["MultiLineText","An multi-line text element. The `Text` element allows only single line text and the `MultiLineText` supports drawing multiple lines"],["Palette100","The palette of 100% accessibility"],["Palette99","The palette of 99% accessibility"],["Palette9999","The palette of 99.99% accessibility"],["PaletteColor","A color in the given palette"],["PathElement","An element of a series of connected lines"],["Pixel","An element of a single pixel"],["Polygon","An element of a filled polygon"],["Quartiles","The quartiles"],["RGBColor","The color described by its RGB value"],["RangedCoord","The coordinate described by two ranged value"],["RangedCoordf32","The ranged coordinate for type f32"],["RangedCoordf64","The ranged coordinate for type f64"],["RangedCoordi32","The ranged coordinate for type i32"],["RangedCoordi64","The ranged coordinate for type i64"],["RangedCoordu32","The ranged coordinate for type u32"],["RangedCoordu64","The ranged coordinate for type u64"],["Rectangle","A rectangle element"],["ShapeStyle","Style for any of shape"],["Text","A single line text element. This can be owned or borrowed string, dependents on `String` or `str` moved into."],["TextStyle","Style of a text"],["TriangleMarker","Describe a triangle marker"]],"trait":[["AsRelative","Allows a value turns into a relative size"],["Color","Any color representation"],["IntoCentric","The trait for types that can decorated by `CentricDiscreteRange` decorator"],["IntoDynElement","The trait that makes the conversion from the statically dispatched element to the dynamically dispatched element"],["IntoFont","The trait that allows some type turns into a font description"],["IntoPartialAxis","The trait for the types that can be converted into a partial axis"],["LogScalable","The trait for the type that is able to be presented in the log scale"],["Palette",""],["Ranged","The trait that indicates we have a ordered and ranged value Which is used to describe the axis"],["SimpleColor","Color without alpha channel"],["ToGroupByRange","The trait that provides method `Self::group_by` function which creates a `GroupBy` decorated ranged value."]],"type":[["DrawResult","The type used to returns a drawing operation that can be failed"]]});