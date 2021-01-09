// HighlightJS for source code prettyfication
//var hljs = require("highlight.js");
var hljs = require("highlight.js/lib/core");
hljs.registerLanguage("rust", require("highlight.js/lib/languages/rust"));
hljs.registerLanguage("json", require("highlight.js/lib/languages/json"));
hljs.registerLanguage("python", require("highlight.js/lib/languages/python"));
hljs.registerLanguage("bash", require("highlight.js/lib/languages/bash"));
hljs.registerLanguage("javascript", require("highlight.js/lib/languages/javascript"));
hljs.registerLanguage("yaml", require("highlight.js/lib/languages/yaml"));
hljs.registerLanguage("xml", require("highlight.js/lib/languages/xml"));

hljs.initHighlightingOnLoad();

