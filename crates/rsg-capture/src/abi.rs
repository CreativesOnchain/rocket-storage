//! Solidity ABI interface bindings for reading RocketStorage state.

use alloy::sol;

sol! {
    #[allow(missing_docs)]
    interface IRocketStorageRead {
        function getAddress(bytes32 _key) external view returns (address);
        function getUint(bytes32 _key) external view returns (uint256);
        function getBool(bytes32 _key) external view returns (bool);
        function getBytes32(bytes32 _key) external view returns (bytes32);
        function getInt(bytes32 _key) external view returns (int256);
        function getString(bytes32 _key) external view returns (string memory);
        function getBytes(bytes32 _key) external view returns (bytes memory);
    }
}
