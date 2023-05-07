pub const SELECTOR_UNI: [&str; 3] = [
    "24856bc3", // "execute(bytes,bytes[])"
    "3593564c", // "execute(bytes,bytes[],uint256)"
    "fa461e33"  // uniswapV3SwapCallback(int256,int256,bytes)
];

pub const SELECTOR_V3_R1: [&str; 1] = ["ac9650d8"]; // "multicall(bytes[])"

pub const SELECTOR_V3_R2: [&str; 5] = [
    "1f0464d1", // "multicall(bytes32,bytes[])"
    "5ae401dc", // "multicall(uint256,bytes[])"
    "ac9650d8", // "multicall(bytes[])"
    "472b43f3", // "swapExactTokensForTokens(uint256,uint256,address[],address)"
    "42712a67", // "swapTokensForExactTokens(uint256,uint256,address[],address)"
];

pub const SELECTOR_V2_R1: [&str; 6] = [
    "fb3bdb41", // "swapETHForExactTokens(uint256,address[],address,uint256)"
    "7ff36ab5", // "swapExactETHForTokens(uint256,address[],address,uint256)"
    "18cbafe5", // "swapExactTokensForETH(uint256,uint256,address[],address,uint256)"
    "38ed1739", // "swapExactTokensForTokens(uint256,uint256,address[],address,uint256)"
    "4a25d94a", // "swapTokensForExactETH(uint256,uint256,address[],address,uint256)"
    "8803dbee", // "swapTokensForExactTokens(uint256,uint256,address[],address,uint256)"
];

pub const SELECTOR_V2_R2: [&str; 9] = [
    "fb3bdb41", // "swapETHForExactTokens(uint256,address[],address,uint256)"
    "7ff36ab5", // "swapExactETHForTokens(uint256,address[],address,uint256)"
    "b6f9de95", // "swapExactETHForTokensSupportingFeeOnTransferTokens(uint256,address[],address,uint256)"
    "18cbafe5", // "swapExactTokensForETH(uint256,uint256,address[],address,uint256)"
    "791ac947", // "swapExactTokensForETHSupportingFeeOnTransferTokens(uint256,uint256,address[],address,uint256)"
    "38ed1739", // "swapExactTokensForTokens(uint256,uint256,address[],address,uint256)"
    "5c11d795", // "swapExactTokensForTokensSupportingFeeOnTransferTokens(uint256,uint256,address[],address,uint256)"
    "4a25d94a", // "swapTokensForExactETH(uint256,uint256,address[],address,uint256)"
    "8803dbee", // "swapTokensForExactTokens(uint256,uint256,address[],address,uint256)"
];

pub const DAI_ADDRESS: &str = "0x6B175474E89094C44Da98b954EedeAC495271d0F";
pub const USDC_ADDRESS: &str = "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48";
pub const USDT_ADDRESS: &str = "0xdAC17F958D2ee523a2206206994597C13D831ec7";
pub const WETH_ADDRESS: &str = "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2";
pub const NULL_ADDRESS: &str = "0x0000000000000000000000000000000000000000";

pub const UNISWAP_UNIVERSAL_ROUTER: &str = "0xEf1c6E67703c7BD7107eed8303Fbe6EC2554BF6B";
pub const UNISWAP_V3_ROUTER_1: &str = "0xE592427A0AEce92De3Edee1F18E0157C05861564";
pub const UNISWAP_V3_ROUTER_2: &str = "0x68b3465833fb72A70ecDF485E0e4C7bD8665Fc45";
pub const UNISWAP_V2_ROUTER_1: &str = "0xf164fC0Ec4E93095b804a4795bBe1e041497b92a";
pub const UNISWAP_V2_ROUTER_2: &str = "0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D";
pub const ONE_INCH_V4_AGG_ROUTER: &str = "0x1111111254fb6c44bAC0beD2854e76F90643097d";
pub const SUSHI_ROUTER: &str = "0xd9e1cE17f2641f24aE83637ab66a2cca9C378B9F";

pub const UNISWAP_V2_FACTORY: &str = "0x5C69bEe701ef814a2B6a3EDD4B1652CB9cc5aA6f";
pub const UNISWAP_V3_FACTORY: &str = "0x1F98431c8aD98523631AE4a59f267346ea31F984";
pub const SUSHI_FACTORY: &str = "0xC0AEe478e3658e2610c5F7A4A2E1777cE9e4f2Ac";

pub const SUSHI_WETH_USDT_LP: &str = "0x06da0fd433C1A5d7a4faa01111c044910A184553";
pub const UNISWAP_V2_WETH_USDT_LP: &str = "0x06da0fd433C1A5d7a4faa01111c044910A184553";
pub const UNISWAP_V3_WETH_USDT_LP_0_01: &str = "0xc7bBeC68d12a0d1830360F8Ec58fA599bA1b0e9b";
pub const UNISWAP_V3_WETH_USDT_LP_0_05: &str = "0x11b815efB8f581194ae79006d24E0d814B7697F6";
pub const UNISWAP_V3_WETH_USDT_LP_0_3: &str = "0x4e68Ccd3E89f51C3074ca5072bbAC773960dFa36";
pub const UNISWAP_V3_WETH_USDT_LP_1: &str = "0xC5aF84701f98Fa483eCe78aF83F11b6C38ACA71D";

pub const SUSHI_WETH_USDC_LP: &str = "0x397FF1542f962076d0BFE58eA045FfA2d347ACa0";
pub const UNISWAP_V2_WETH_USDC_LP: &str = "0xB4e16d0168e52d35CaCD2c6185b44281Ec28C9Dc";
pub const UNISWAP_V3_WETH_USDC_LP_0_01: &str = "0xE0554a476A092703abdB3Ef35c80e0D76d32939F";
pub const UNISWAP_V3_WETH_USDC_LP_0_05: &str = "0x88e6A0c2dDD26FEEb64F039a2c41296FcB3f5640";
pub const UNISWAP_V3_WETH_USDC_LP_0_3: &str = "0x8ad599c3A0ff1De082011EFDDc58f1908eb6e6D8";
pub const UNISWAP_V3_WETH_USDC_LP_1: &str = "0x7BeA39867e4169DBe237d55C8242a8f2fcDcc387";

pub const SUSHI_USDT_USDC_LP: &str = "0xD86A120a06255Df8D4e2248aB04d4267E23aDfaA";
pub const UNISWAP_V2_USDT_USDC_LP: &str = "0x3041CbD36888bECc7bbCBc0045E3B1f144466f5f";
pub const UNISWAP_V3_USDT_USDC_LP_0_01: &str = "0x3416cF6C708Da44DB2624D63ea0AAef7113527C6";
pub const UNISWAP_V3_USDT_USDC_LP_0_05: &str = "0x7858E59e0C01EA06Df3aF3D20aC7B0003275D4Bf";
pub const UNISWAP_V3_USDT_USDC_LP_0_3: &str = "0xEe4Cf3b78A74aFfa38C6a926282bCd8B5952818d";
pub const UNISWAP_V3_USDT_USDC_LP_1: &str = "0xbb256c2F1B677e27118b0345FD2b3894D2E6D487";