/**
 * date:2019/08/16
 * author:Mr.Chung
 * description:此处放layui自定义扩展
 */

window.rootPath = (function (src) {
    src = document.scripts[document.scripts.length - 1].src;
    return src.substring(0, src.lastIndexOf("/") + 1);
})();

layui.config({
    base: rootPath + "lay-module/",
    version: true
}).extend({
    miniAdmin: "layuimini/miniAdmin", // layuimini后台扩展
    miniMenu: "layuimini/miniMenu", // layuimini菜单扩展
    miniPage: "layuimini/miniPage", // layuimini 单页扩展
    miniTheme: "layuimini/miniTheme", // layuimini 主题扩展
    xmSelect: 'xm-select/xm-select',// 多选下拉框
    tataUtil: 'tatautil/tatautil',
    eleTree: 'ele-tree/eleTree', // 树形控件
    imageupload: 'imageupload/imageupload', // 上传图片封装
});

// 加载非layui模块js文件
layui.extend({
    simplemde: 'simplemde/simplemde.min',
    jqtree: 'jqtree/tree.jquery'
});

layui.define(function(exports){
    exports('simplemde', null);
});

layui.define(function(exports) {
    exports('jqtree', null);
});