layui.define(['upload','layer'],function(exports){
    var upload = layui.upload,
        layer = layui.layer;
    var uploader = {
        render: function(elem) {
            upload.render({
                elem: elem,
                url: '/admin/image/upload/',
                accept: 'images',
                acceptMime: 'image/*',
                size: 10 * 1024,
                multiple: false,
                before: function() {
                    layer.load(2, {time: 5*1000});
                },
                done: function(res){
                    layer.closeAll('loading');
                    if (res.Ok != undefined) {
                        layer.open({
                            title: "上传成功",
                            shadeClose: false,
                            content: '\<input type="text" class="layui-input" id="img-src-link" readonly value="' + res.Ok + '">\<\/input>',
                            btn: ['复制链接'],
                            btnAlign: 'c',
                            yes: function(index, layero){
                                layero.context.getElementById("img-src-link").select();
                                document.execCommand("Copy");
                                layer.close(index);
                            }
                        });
                    } else {
                        layer.msg(res.Err, {icon: 2});
                    }
                },
                error: function(){
                    layer.msg("出错了", {icon: 2});
                }
            });
        }
    };

    exports('imageupload', uploader);
})