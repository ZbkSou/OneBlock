1.在 Offchain Worker 中，使用 Offchain Indexing 特性实现从链上向 Offchain Storage 中写入数据
2.使用 js sdk 从浏览器 frontend 获取到前面写入 Offchain Storage 的数据
3.回答链上随机数（如前面Kitties示例中）与链下随机数的区别
链上随机数是根据当前结点的之前81个block的哈希生成的，链上无法实现真正的不可预测的随机数种子来保证生成的随机数真实随机
链下随机数是在链下执行，可以使用当前结点系统关联生成不可预测的随机数种子，以确保生成数的随机性。
4.（可选）在 Offchain Worker 中，解决向链上发起不签名请求时剩下的那个错误。参考：https://github.com/paritytech/substrate/blob/master/frame/examples/offchain-worker/src/lib.rs
5.（可选）构思一个应用场景，描述如何使用 Offchain Features 三大组件去实现它

创造小猫之后将 id 传入 Offchain index ，Offchain Worker 取出做复杂运算，将符合条件的小猫id 调用签名方法修改DNA
6.（可选）如果有时间，可以实现一个上述原型

![图片](https://github.com/ZbkSou/OneBlock/blob/master/lesson04/1664542962432.png)
![图片](https://github.com/ZbkSou/OneBlock/blob/master/lesson04/1664550403284.png)
