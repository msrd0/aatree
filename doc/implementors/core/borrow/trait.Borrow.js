(function() {var implementors = {};
implementors["bstr"] = [{"text":"impl Borrow&lt;BStr&gt; for BString","synthetic":false,"types":[]}];
implementors["crossbeam_epoch"] = [{"text":"impl&lt;T:&nbsp;?Sized + Pointable&gt; Borrow&lt;T&gt; for Owned&lt;T&gt;","synthetic":false,"types":[]}];
implementors["plotters"] = [{"text":"impl&lt;'a, DB:&nbsp;DrawingBackend, CT1:&nbsp;CoordTranslate, CT2:&nbsp;CoordTranslate&gt; Borrow&lt;ChartContext&lt;'a, DB, CT1&gt;&gt; for DualCoordChartContext&lt;'a, DB, CT1, CT2&gt;","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()