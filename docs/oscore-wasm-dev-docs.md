# Oscore Wasm 文档

## Wasm算法参数的设定
Oscore支持使用Rust语言编写Wasm算法。算法的输入参数是用户的代币余额，币天等信息，然后计算出相应的score值返回。
由于不同的算法对输入参数的要求可能不同，因此需要在部署算法时指定相关的请求参数。
目前支持的请求参数如下：
```json
 {
	"tokens": ["ONT", "ETH"], //["ALL"],
	"queryItems" : {
		"balance": bool,
		"xday_sum": 10,
	}
}
```
tokens表示算法需要的代币列表，比如算法如果只关心用户的BTC和ETH相关信息，那么则可以填["BTC", "ETH"]。另外ALL表示需要所有的代币。

queryItems表示算法需要查询代币的那些信息：balance表示是否需要查询余额，xday_sum表示多少币天的信息。后续可能还会扩展其他字段。

当指定参数后，当要计算用户的score时，服务器会根据指定的参数将用户的相关数据准备好，然后作为输入参数提交给算法。上面的参数将会给算法提供如下的json输入：
```json
{
	"asset_infos": [{
		"token_name": 'ONT",
		"balance": "1000000", 
		"xday_sum": {"amount": "10000000000", days: 10}
	},{
		"token_name": 'ETH",
		"balance": "100", 
		"xday_sum": {"amount": "10000", days: 10}
	}],
	user_did: "ont:id:xxx"
}
```
算法根据输入计算出一个整数值作为score分，并返回一个 { score: uint32} 的json结构体给服务器。

## 示例

下面的例子根据用户的ONT余额计算score分值。分值根据ONT余额线性递增至10万为止。
请求参数设置为：
```json
{
	"tokens": ["ONT"], 
	"queryItems" : {"balance": true}
}
```

rust代码如下：

```rust
use oscore::runtime;
use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
pub struct TokenInfo {
    token_name: String,
    balance: String,
}

#[derive(Serialize, Deserialize)]
pub struct AssetInfoData {
    asset_infos: Vec<TokenInfo>,
    user_did :String,
}

#[derive(Serialize, Deserialize)]
pub struct ScoreResult {
    score: u32,
}

fn calculate_score(assets: &AssetInfoData) -> u32 {
    let ont_balance: u64 = assets.asset_infos.iter().find(|info| info.token_name=="ONT")
		.map(|info| info.balance.parse().expect("balance format error"))
		.unwrap_or_default();
    let score = ont_balance / 10_0000;
    let score = std::cmp::max(score, 100);
    
    score as u32
}

#[no_mangle]
pub fn invoke() {
    oscore::set_panic_handler();
    let input = runtime::input();
    let assets: AssetInfoData = serde_json::from_slice(input.as_slice()).expect("decode input failed");
	let score = calculate_score(&assets);
    let score = ScoreResult { score: score};
    let result = serde_json::to_string(&score).unwrap();
    runtime::ret(result.as_ref());
}
```

