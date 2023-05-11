# つむぎエンジン（TsumugiEngine）
つむぎエンジンはつむぎライブラリを用いて作ったエンジンです。
非同期、並列、疎な依存性を持つエンジンを目指して作りました。
## 内部ライブラリ群
### tsumugi
つむぎライブラリ。イベント管理やオブジェクト管理を非同期で行う、以下のすべてのライブラリが用いるライブラリ。

基本的にはつむぎライブラリを複数動作させることで、エンジンが構成されている。
また、それぞれのライブラリは異なる周期で独立に動作しているため、tsumugiを通してライブラリ間のデータの受け渡しを行う。
***


### tsumuFigureStockCPU
3Dオブジェクトの頂点情報やボーンの状態などを管理するライブラリ
### tsumugiShaderStock
シェーダーやマテリアル情報を管理するライブラリ
### tsumuObject
オブジェクトの場所や状態を管理するライブラリ
### tsumugiSimpleHashMap
ハッシュマップと配列を組み合わせることで、マテリアルやオブジェクを素早く回せるようなリストが記述されているライブラリ
### tsumuGraphic_DirectX12
DirectX12を用いた描画エンジン（Windows専用）
### tsumugiKeyboardInput
キーボード操作を管理するライブラリ（Windows専用）
### tsumugiWindowController
ウィンドウを生成したり管理するようなライブラリ（Windows専用）
### tsumuDebugwin
imguiを用いたデバッグ用ライブラリ（Windows専用）
内部でvcpkgを用いているため、[cargo-vcpkg](https://crates.io/crates/cargo-vcpkg "cargo-vcpkg")に依存
![TumuScreenshot](https://user-images.githubusercontent.com/43674314/208431873-b2db91d0-07bd-460e-b483-75030d12a6c6.png)

![ページ_18](https://user-images.githubusercontent.com/43674314/223162347-c51cd7bf-66c2-432f-932f-b2ea1fda4718.png)

![ページ_19](https://user-images.githubusercontent.com/43674314/223162355-d1108f5a-8578-479a-90ac-415143cfdf74.png)
![ページ_20](https://user-images.githubusercontent.com/43674314/223162366-4110a19a-0273-43ff-8b9f-04eff662a1a2.png)
![ページ_21](https://user-images.githubusercontent.com/43674314/223162378-00c2347e-a929-4c17-970a-da9398aa3384.png)
![ページ_22](https://user-images.githubusercontent.com/43674314/223162391-2c0a7d67-2de3-43b3-9785-68cb2d00f415.png)

#### レンダラーごとに条件を変えることができるため、複数の条件を一度に比較することが出来る
##### カメラの場所のみを変えて同時にレンダリング
![ページ_23](https://github.com/snamas/TsumugiEngine/assets/43674314/e66ed779-5718-463b-a390-e333888f2007)

##### ディスプレイの解像度のみを変えて同時にレンダリング
![ページ_24](https://github.com/snamas/TsumugiEngine/assets/43674314/a2d0af31-3150-41c1-9982-d4ab7a7a3616)
