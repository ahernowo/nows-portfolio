// Makes the editor's preview pane look like the real site.
/* global CMS, createClass, h */
(function () {
  if (typeof CMS === "undefined") return;
  CMS.registerPreviewStyle("/main.css");
  CMS.registerPreviewStyle("/giallo.css");

  var ArticlePreview = createClass({
    render: function () {
      var entry = this.props.entry;
      return h(
        "article",
        { className: "article" },
        h(
          "header",
          { className: "article-head wrap-text" },
          h("h1", { className: "article-title" }, entry.getIn(["data", "title"])),
          h("p", { className: "article-dek" }, entry.getIn(["data", "description"]))
        ),
        h("div", { className: "prose wrap-text" }, this.props.widgetFor("body"))
      );
    },
  });

  var WorkPreview = createClass({
    render: function () {
      var entry = this.props.entry;
      return h(
        "article",
        { className: "case wrap" },
        h(
          "header",
          { className: "case-head" },
          h("h1", { className: "case-title" }, entry.getIn(["data", "title"])),
          h("p", { className: "case-dek" }, entry.getIn(["data", "description"]))
        ),
        h("div", { className: "prose case-body" }, this.props.widgetFor("body"))
      );
    },
  });

  CMS.registerPreviewTemplate("articles", ArticlePreview);
  CMS.registerPreviewTemplate("work", WorkPreview);
})();
