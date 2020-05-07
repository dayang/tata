layui.define(function(exports){
    var util = {
        getQueryParam: function(url, name) {
            var reg = new RegExp("(^|&|\\?)" + name + "=([^&]*)(&|$)");
            var r = url.match(reg);
            if (r != null) return encodeURI(r[2]); return null;
        },
        
    }

    exports('tataUtil', util);
})