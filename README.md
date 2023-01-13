# つむぎエンジン（TsumugiEngine）
つむぎエンジンはつむぎライブラリを用いて作ったエンジンです。
非同期、並列、疎な依存性を持つエンジンを目指して作りました。
## 内部ライブラリ群
### tsumugi
つむぎライブラリ。イベント管理やオブジェクト管理を非同期で行う、以下のすべてのライブラリが用いるライブラリ。

基本的にはつむぎライブラリを複数動作させることで、エンジンが構成されている。
また、それぞれのライブラリは異なる周期で並列に動作しているため、tsumugiを通してライブラリ間のデータの受け渡しを行う。
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
