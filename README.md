# つむぎエンジン（TsumugiEngine）
つむぎエンジンは[つむぎ](https://github.com/snamas/tsumugi/blob/main/README.md "つむぎ")ライブラリを用いて作ったエンジンです。
非同期、並列、疎な依存性を持つエンジンを目指して作りました。
## 内部ライブラリ群

### tsumuFigureStockCPU
3Dオブジェクトの頂点情報やボーンの状態などを管理するライブラリ
### tsumugiShaderStock
シェーダーやマテリアル情報を管理するライブラリ
### tsumuObject
オブジェクトの場所や状態を管理するライブラリ
### tsumugiSimpleHashMap
ハッシュマップと配列を組み合わせることで、マテリアルやオブジェクを素早く回せるようなHashMapライブラリ
### tsumuGraphic_DirectX12
DirectX12を用いた描画エンジン（Windows専用）
### tsumugiKeyboardInput
キーボード操作を管理するライブラリ（Windows専用）
### tsumugiWindowController
ウィンドウを生成したり管理するようなライブラリ（Windows専用）
### tsumuDebugwin
imguiを用いたデバッグ用ライブラリ（Windows専用）
