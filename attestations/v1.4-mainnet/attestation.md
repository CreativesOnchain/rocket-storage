# Rocket Pool RocketStorage Upgrade Attestation

**Upgrade:** Rocket Pool v1.4 / Saturn 1  
**Verdict:** `PASS`

> **✅ PASS** — All required RocketStorage effects and allowed external calls match the reviewed manifest exactly.

## Pinned Parameters

| Field | Value |
|---|---|
| Chain ID | `1` |
| Pre-upgrade block | `24479993` |
| Pre-block hash | `0x8c26a07a5b2c987c58afd5e3458115a5ce49e6e6e9cff72780d2dfa7b89f3bfd` |
| Upgrade transaction | `0x2fc10aad3c1b00bdfa9b6fddab79e0f2688609848f8f7a1a6449ab42da38530c` |
| Exec block | `24479994` |
| Upgrade contract | `0x5b3b5c76391662e56d0ff72f31b89c409316c8ba` |
| RocketStorage | `0x1d8f8f00cfa6758d7be78336684788fb0ee0fa46` |
| Source commit | `fb7d9c428dc3dddc3fbd3e634e3cb365655df89e` |

## Input Hashes

| Artifact | SHA-256 |
|---|---|
| Observed trace | `6212d49dad9cab7c60132bcdfef43bc0a24629b0b61fef0d5ffa55dd6705c757` |
| Manifest | `40e13752aca6517d61af48a66fe4567ab246c4fa3b23dbd8a3351d1ab3456063` |
| Review record | `50815876c5b5772a0d5e0c0c48e6f47ffff44d0df4af26606073ae05d439b312` |
| Tool version | `rsg/0.1.0` |

## Observed RocketStorage Effects

| # | Op | Key | Old Value | New Value |
|---|---|---|---|---|
| 5 | `SetBool` | `contract.exists.rocketMegapoolDelegate` | `false` | `true` |
| 6 | `SetString` | `contract.name.rocketMegapoolDelegate` | `` | `rocketMegapoolDelegate` |
| 7 | `SetAddress` | `contract.address.rocketMegapoolDelegate` | `0x0000000000000000000000000000000000000000` | `0xca3dd4bee7c174903dbf66c3897c27e9adaaebdd` |
| 8 | `SetString` | `contract.abi.rocketMegapoolDelegate` | `` | `eJztWt1v2zgM/1cOfi7uYXe3h71lbTcE6Hpd2t49DE…` |
| 12 | `SetBool` | `contract.exists.rocketMegapoolFactory` | `false` | `true` |
| 13 | `SetString` | `contract.name.rocketMegapoolFactory` | `` | `rocketMegapoolFactory` |
| 14 | `SetAddress` | `contract.address.rocketMegapoolFactory` | `0x0000000000000000000000000000000000000000` | `0xd5bffeaa9f373b9c367132772faa0b88e3f0e38b` |
| 15 | `SetString` | `contract.abi.rocketMegapoolFactory` | `` | `eJzNlEFLxDAQhf+K5NyToCx7E+vBwyKot2WRaTJbgn…` |
| 19 | `SetBool` | `contract.exists.rocketMegapoolProxy` | `false` | `true` |
| 20 | `SetString` | `contract.name.rocketMegapoolProxy` | `` | `rocketMegapoolProxy` |
| 21 | `SetAddress` | `contract.address.rocketMegapoolProxy` | `0x0000000000000000000000000000000000000000` | `0x1b389d76a04d01026c5f5b0a125d4ccf26f9cd51` |
| 22 | `SetString` | `contract.abi.rocketMegapoolProxy` | `` | `eJzNlE1PwzAMhv8KyrknJDjshsQOSHAZ7DShKU3cES…` |
| 26 | `SetBool` | `contract.exists.rocketMegapoolManager` | `false` | `true` |
| 27 | `SetString` | `contract.name.rocketMegapoolManager` | `` | `rocketMegapoolManager` |
| 28 | `SetAddress` | `contract.address.rocketMegapoolManager` | `0x0000000000000000000000000000000000000000` | `0xf2ccd522ba5ffeda28fe0389963845d61f342034` |
| 29 | `SetString` | `contract.abi.rocketMegapoolManager` | `` | `eJztWVFP2zAQ/itTnivEOoYQb8DYVAkQA7Y9IITc5J…` |
| 33 | `SetBool` | `contract.exists.linkedListStorage` | `false` | `true` |
| 34 | `SetString` | `contract.name.linkedListStorage` | `` | `linkedListStorage` |
| 35 | `SetAddress` | `contract.address.linkedListStorage` | `0x0000000000000000000000000000000000000000` | `0x52590e8aac140e2020f8f51695719922ebccb6d6` |
| 36 | `SetString` | `contract.abi.linkedListStorage` | `` | `eJztWFtP2zAU/isozxEPoE2It0l7WLWOyzbxUlWVa5…` |
| 40 | `SetBool` | `contract.exists.rocketDAOProtocolSettingsMegapool` | `false` | `true` |
| 41 | `SetString` | `contract.name.rocketDAOProtocolSettingsMegapool` | `` | `rocketDAOProtocolSettingsMegapool` |
| 42 | `SetAddress` | `contract.address.rocketDAOProtocolSettingsMegapool` | `0x0000000000000000000000000000000000000000` | `0x40628faac22383327b9f7bbc86cd1857050a2dce` |
| 43 | `SetString` | `contract.abi.rocketDAOProtocolSettingsMegapool` | `` | `eJzNlk9PAjEQxb+K6XlPJhrCTYImJmKIQDwQQoZllp…` |
| 47 | `SetBool` | `contract.exists.rocketDAOSecurityUpgrade` | `false` | `true` |
| 48 | `SetString` | `contract.name.rocketDAOSecurityUpgrade` | `` | `rocketDAOSecurityUpgrade` |
| 49 | `SetAddress` | `contract.address.rocketDAOSecurityUpgrade` | `0x0000000000000000000000000000000000000000` | `0x950baf0358164339114914169bf16754789b5dc4` |
| 50 | `SetString` | `contract.abi.rocketDAOSecurityUpgrade` | `` | `eJy9k8FqwzAMhl9l+JzTYKP0Ntilh8Loyi6lDMVRg6…` |
| 54 | `SetBool` | `contract.exists.rocketNetworkRevenues` | `false` | `true` |
| 55 | `SetString` | `contract.name.rocketNetworkRevenues` | `` | `rocketNetworkRevenues` |
| 56 | `SetAddress` | `contract.address.rocketNetworkRevenues` | `0x0000000000000000000000000000000000000000` | `0x9d9708da8e0200dd8dd9ad09e0aaf184ad260842` |
| 57 | `SetString` | `contract.abi.rocketNetworkRevenues` | `` | `eJzVVU1vwjAM/StTzpymDSFuiF12GJso4oIQMsFAtJ…` |
| 61 | `SetBool` | `contract.exists.beaconStateVerifier` | `false` | `true` |
| 62 | `SetString` | `contract.name.beaconStateVerifier` | `` | `beaconStateVerifier` |
| 63 | `SetAddress` | `contract.address.beaconStateVerifier` | `0x0000000000000000000000000000000000000000` | `0xe9a114c50f26001443b91079ab5573a90d2d8469` |
| 64 | `SetString` | `contract.abi.beaconStateVerifier` | `` | `eJztlctu2zAQRX8l0NoLx02MIru2CFIvWgRx0C4Mox…` |
| 68 | `SetBool` | `contract.exists.rocketMegapoolPenalties` | `false` | `true` |
| 69 | `SetString` | `contract.name.rocketMegapoolPenalties` | `` | `rocketMegapoolPenalties` |
| 70 | `SetAddress` | `contract.address.rocketMegapoolPenalties` | `0x0000000000000000000000000000000000000000` | `0xa2afc3c2d8ea4ebdbe925cade17c29517630e6ab` |
| 71 | `SetString` | `contract.abi.rocketMegapoolPenalties` | `` | `eJztVMFqwkAQ/ZWy55xKK+JNPPUgFJVeRGSSTMLSZD…` |
| 75 | `SetBool` | `contract.exists.rocketNetworkSnapshotsTime` | `false` | `true` |
| 76 | `SetString` | `contract.name.rocketNetworkSnapshotsTime` | `` | `rocketNetworkSnapshotsTime` |
| 77 | `SetAddress` | `contract.address.rocketNetworkSnapshotsTime` | `0x0000000000000000000000000000000000000000` | `0x569f5b3024054ab4049a50df223a747afe18a891` |
| 78 | `SetString` | `contract.abi.rocketNetworkSnapshotsTime` | `` | `eJztVstOwzAQ/BXkcy4UqEpviBMHLqXiUlWR425bK6…` |
| 81 | `SetBool` | `contract.exists.rocketNodeManager` | `false` | `true` |
| 82 | `SetString` | `contract.name.rocketNodeManager` | `` | `rocketNodeManager` |
| 83 | `SetAddress` | `contract.address.rocketNodeManager` | `0x2b52479f6ea009907e46fc43e91064d1b92fdc86` | `0xcf2d76a7499d3acb5a22ce83c027651e8d76e250` |
| 84 | `SetString` | `contract.abi.rocketNodeManager` | `eJzVWltv2zYU/iuDn4OB4lXqW5d2WIE0C5IUeyiK4J…` | `eJzdWVFv2jAQ/itTntEeJm2q+lZ1nVapRajQ7aFCyC…` |
| 85 | `DeleteString` | `contract.name.old.rocketNodeManager` | `rocketNodeManager` | `` |
| 86 | `DeleteBool` | `contract.exists.old.rocketNodeManager` | `true` | `false` |
| 89 | `SetBool` | `contract.exists.rocketNodeDeposit` | `false` | `true` |
| 90 | `SetString` | `contract.name.rocketNodeDeposit` | `` | `rocketNodeDeposit` |
| 91 | `SetAddress` | `contract.address.rocketNodeDeposit` | `0x672335b91b4f2096d897ca1b12ef4ec9346a5ff4` | `0x6b13698c306a297fee1383cdc2c65d63781d2d47` |
| 92 | `SetString` | `contract.abi.rocketNodeDeposit` | `eJztWFFv2zYQ/iuDnv1ASqRE5i1bW3QP2Ya0Sx+CID…` | `eJztWE1v2zAM/SuDz0EPHTYUuaVrhu3QbUiz7hAEAW…` |
| 93 | `DeleteString` | `contract.name.old.rocketNodeDeposit` | `rocketNodeDeposit` | `` |
| 94 | `DeleteBool` | `contract.exists.old.rocketNodeDeposit` | `true` | `false` |
| 97 | `SetBool` | `contract.exists.rocketNodeStaking` | `false` | `true` |
| 98 | `SetString` | `contract.name.rocketNodeStaking` | `` | `rocketNodeStaking` |
| 99 | `SetAddress` | `contract.address.rocketNodeStaking` | `0xf18dc176c10ff6d8b5a17974126d43301f8eeb95` | `0xedfc7dcae43ff954577a2875a9d805874490ee3e` |
| 100 | `SetString` | `contract.abi.rocketNodeStaking` | `eJztWVtv2zYU/iuDn/NA8vCat27YsAHtUKTZ9lAUwS…` | `eJztWd9v2jAQ/lemPPM0aVPFG502bRJME9DtoULoSA…` |
| 101 | `DeleteString` | `contract.name.old.rocketNodeStaking` | `rocketNodeStaking` | `` |
| 102 | `DeleteBool` | `contract.exists.old.rocketNodeStaking` | `true` | `false` |
| 105 | `SetBool` | `contract.exists.rocketDepositPool` | `false` | `true` |
| 106 | `SetString` | `contract.name.rocketDepositPool` | `` | `rocketDepositPool` |
| 107 | `SetAddress` | `contract.address.rocketDepositPool` | `0xdd3f50f8a6cafbe9b31a427582963f465e745af8` | `0xce15294273cfb9d9b628f4d61636623decdf4fdc` |
| 108 | `SetString` | `contract.abi.rocketDepositPool` | `eJztl8tu2zoQhl/lQGsveL9kl9MUaIG2i97OogiKIT…` | `eJztWMlu2zAQ/ZVCZ59StAhyy4oGSIo2cdtDYBhjce…` |
| 109 | `DeleteString` | `contract.name.old.rocketDepositPool` | `rocketDepositPool` | `` |
| 110 | `DeleteBool` | `contract.exists.old.rocketDepositPool` | `true` | `false` |
| 113 | `SetBool` | `contract.exists.rocketDAOProtocol` | `false` | `true` |
| 114 | `SetString` | `contract.name.rocketDAOProtocol` | `` | `rocketDAOProtocol` |
| 115 | `SetAddress` | `contract.address.rocketDAOProtocol` | `0x1b714ed0ce30a8bedc5b4253daaa08c84ca5bfcb` | `0xcac25e88276a333cf9d4196d112d93af67ef809a` |
| 116 | `SetString` | `contract.abi.rocketDAOProtocol` | `eJztWttu20YQ/ZVCz37Y+yVvuRRF0Dg1muQpCIzZ3V…` | `eJztWltP2zAU/isoz32atAnxxmWa0CirBjwhFDnJaW…` |
| 117 | `DeleteString` | `contract.name.old.rocketDAOProtocol` | `rocketDAOProtocol` | `` |
| 118 | `DeleteBool` | `contract.exists.old.rocketDAOProtocol` | `true` | `false` |
| 121 | `SetBool` | `contract.exists.rocketDAOProtocolProposals` | `false` | `true` |
| 122 | `SetString` | `contract.name.rocketDAOProtocolProposals` | `` | `rocketDAOProtocolProposals` |
| 123 | `SetAddress` | `contract.address.rocketDAOProtocolProposals` | `0x6d736da1dc2562dbea9998385a0a27d8c2b2793e` | `0xcf7f6e23cd8189b7f56b14f66e11241c8ac0f03b` |
| 124 | `SetString` | `contract.abi.rocketDAOProtocolProposals` | `eJztWU1v2zgQ/SuFzznwS/zobdu9BLvtFv04FUUw5A…` | `eJztWU1P4zAQ/Sso555W2hXitsteqqVsVeBUVZXrTI…` |
| 125 | `DeleteString` | `contract.name.old.rocketDAOProtocolProposals` | `rocketDAOProtocolProposals` | `` |
| 126 | `DeleteBool` | `contract.exists.old.rocketDAOProtocolProposals` | `true` | `false` |
| 129 | `SetBool` | `contract.exists.rocketDAOProtocolSettingsNode` | `false` | `true` |
| 130 | `SetString` | `contract.name.rocketDAOProtocolSettingsNode` | `` | `rocketDAOProtocolSettingsNode` |
| 131 | `SetAddress` | `contract.address.rocketDAOProtocolSettingsNode` | `0x448da008c7eb2501165c9aa62dffeec4405bc660` | `0xb02b883303e658ddcd58d3871dc4ca0c91f0fc9d` |
| 132 | `SetString` | `contract.abi.rocketDAOProtocolSettingsNode` | `eJzNlk1v4jAQhv/KKmcOScgX3LbaXWkPSFXZ7aWqqr…` | `eJzdlk1rwkAQhv9KydlToaV4q/2AgoWi/TiIlHEzjY…` |
| 133 | `DeleteString` | `contract.name.old.rocketDAOProtocolSettingsNode` | `rocketDAOProtocolSettingsNode` | `` |
| 134 | `DeleteBool` | `contract.exists.old.rocketDAOProtocolSettingsNode` | `true` | `false` |
| 137 | `SetBool` | `contract.exists.rocketDAOProtocolSettingsDeposit` | `false` | `true` |
| 138 | `SetString` | `contract.name.rocketDAOProtocolSettingsDeposit` | `` | `rocketDAOProtocolSettingsDeposit` |
| 139 | `SetAddress` | `contract.address.rocketDAOProtocolSettingsDeposit` | `0xd846aa34caef083dc4797d75096f60b6e08b7418` | `0x227be8dd01df8ad9bed0178e4f8cec2996c5c365` |
| 140 | `SetString` | `contract.abi.rocketDAOProtocolSettingsDeposit` | `eJzNlk1vm0AQhv9KxdkHwMuHfUvUVOohUlS3pyiKht…` | `eJzVllFPwjAQx7+K2fOeTDSGN4iYmEiCDJ/IQso4Zu…` |
| 141 | `DeleteString` | `contract.name.old.rocketDAOProtocolSettingsDeposit` | `rocketDAOProtocolSettingsDeposit` | `` |
| 142 | `DeleteBool` | `contract.exists.old.rocketDAOProtocolSettingsDeposit` | `true` | `false` |
| 145 | `SetBool` | `contract.exists.rocketDAOProtocolSettingsNetwork` | `false` | `true` |
| 146 | `SetString` | `contract.name.rocketDAOProtocolSettingsNetwork` | `` | `rocketDAOProtocolSettingsNetwork` |
| 147 | `SetAddress` | `contract.address.rocketDAOProtocolSettingsNetwork` | `0x89682e5f9bf69c909fc5e21a06495ac35e3671ab` | `0x67fd03a5095197d1ad1f932bc55e022c420b1153` |
| 148 | `SetString` | `contract.abi.rocketDAOProtocolSettingsNetwork` | `eJzNl11v2jAUhv/KlGsu8v3B3VpWaRedELCrClUn9j…` | `eJzVmE2P2jAQhv9KlTOnSq1We2NhV6q02yKgvSCEhm…` |
| 149 | `DeleteString` | `contract.name.old.rocketDAOProtocolSettingsNetwork` | `rocketDAOProtocolSettingsNetwork` | `` |
| 150 | `DeleteBool` | `contract.exists.old.rocketDAOProtocolSettingsNetwork` | `true` | `false` |
| 153 | `SetBool` | `contract.exists.rocketDAOProtocolSettingsSecurity` | `false` | `true` |
| 154 | `SetString` | `contract.name.rocketDAOProtocolSettingsSecurity` | `` | `rocketDAOProtocolSettingsSecurity` |
| 155 | `SetAddress` | `contract.address.rocketDAOProtocolSettingsSecurity` | `0x1ec364cdd9697f56b8cb17a745b98c2b862cbe29` | `0xc9d771aaf504f33bb3c8a7e67ea9f1881f837cff` |
| 156 | `SetString` | `contract.abi.rocketDAOProtocolSettingsSecurity` | `eJzNlctu2zAQRX+l0NoLSpFk2bsE6KJAC7RN2k0QBE…` | `eJzNlk1Lw0AQhv+K7DknQSm9VfQgVFD7cSmhTNNpXE…` |
| 157 | `DeleteString` | `contract.name.old.rocketDAOProtocolSettingsSecurity` | `rocketDAOProtocolSettingsSecurity` | `` |
| 158 | `DeleteBool` | `contract.exists.old.rocketDAOProtocolSettingsSecurity` | `true` | `false` |
| 161 | `SetBool` | `contract.exists.rocketDAOProtocolSettingsMinipool` | `false` | `true` |
| 162 | `SetString` | `contract.name.rocketDAOProtocolSettingsMinipool` | `` | `rocketDAOProtocolSettingsMinipool` |
| 163 | `SetAddress` | `contract.address.rocketDAOProtocolSettingsMinipool` | `0xa416a7a07925d60f794e20532bc730749611a220` | `0xaef94c3650aa13d7a2456477fc374a16b94b9152` |
| 164 | `SetString` | `contract.abi.rocketDAOProtocolSettingsMinipool` | `eJzNV9tuozAQ/ZWK5zwYMJf0rd3ualdqparp5aGqqs…` | `eJzNV01v4jAQ/StVzpwq7Qr1Vra72kogodKPA0JoQg…` |
| 165 | `DeleteString` | `contract.name.old.rocketDAOProtocolSettingsMinipool` | `rocketDAOProtocolSettingsMinipool` | `` |
| 166 | `DeleteBool` | `contract.exists.old.rocketDAOProtocolSettingsMinipool` | `true` | `false` |
| 169 | `SetBool` | `contract.exists.rocketDAOSecurityProposals` | `false` | `true` |
| 170 | `SetString` | `contract.name.rocketDAOSecurityProposals` | `` | `rocketDAOSecurityProposals` |
| 171 | `SetAddress` | `contract.address.rocketDAOSecurityProposals` | `0x6004fa90a27db9971add200d1a3bb34444db9fb7` | `0x334b9b1a6f9d7531efb13746482ff40f1c2a0c4e` |
| 172 | `SetString` | `contract.abi.rocketDAOSecurityProposals` | `eJzVlk1v2zAMhv/K4HMOrhN/5bZil2DIMDTbKSgGWa…` | `eJzdls9PwjAUx/8VszMnE43hpvFCDMaAnggxXXlgQ2…` |
| 173 | `DeleteString` | `contract.name.old.rocketDAOSecurityProposals` | `rocketDAOSecurityProposals` | `` |
| 174 | `DeleteBool` | `contract.exists.old.rocketDAOSecurityProposals` | `true` | `false` |
| 177 | `SetBool` | `contract.exists.rocketDAONodeTrustedUpgrade` | `false` | `true` |
| 178 | `SetString` | `contract.name.rocketDAONodeTrustedUpgrade` | `` | `rocketDAONodeTrustedUpgrade` |
| 179 | `SetAddress` | `contract.address.rocketDAONodeTrustedUpgrade` | `0x952999ec97248547d810fd6464fdb78855b022ab` | `0x9290aa076a2f1418a4e414e3d83ae03ca8e1ad10` |
| 180 | `SetString` | `contract.abi.rocketDAONodeTrustedUpgrade` | `eJzVVclu20AM/ZVCZx9m4yy+pT3l0EuXUxAUs3AMoY…` | `eJzlV01v2zAM/SuDzsEOHVYUuWXrDjm0K/qxS1EUtM…` |
| 181 | `DeleteString` | `contract.name.old.rocketDAONodeTrustedUpgrade` | `rocketDAONodeTrustedUpgrade` | `` |
| 182 | `DeleteBool` | `contract.exists.old.rocketDAONodeTrustedUpgrade` | `true` | `false` |
| 185 | `SetBool` | `contract.exists.rocketNetworkBalances` | `false` | `true` |
| 186 | `SetString` | `contract.name.rocketNetworkBalances` | `` | `rocketNetworkBalances` |
| 187 | `SetAddress` | `contract.address.rocketNetworkBalances` | `0x6cc65bf618f55ce2433f9d8d827fc44117d81399` | `0x1d9f14c6bfd8358b589964bad8665add248e9473` |
| 188 | `SetString` | `contract.abi.rocketNetworkBalances` | `eJztVk1vozAQ/SsrzjlgsI3JbStV6h720qSnqqrG9p…` | `eJztVk1vgkAQ/SvNnjk1aWO81cTEHnpRPBljBhiUuO…` |
| 189 | `DeleteString` | `contract.name.old.rocketNetworkBalances` | `rocketNetworkBalances` | `` |
| 190 | `DeleteBool` | `contract.exists.old.rocketNetworkBalances` | `true` | `false` |
| 193 | `SetBool` | `contract.exists.rocketNetworkSnapshots` | `false` | `true` |
| 194 | `SetString` | `contract.name.rocketNetworkSnapshots` | `` | `rocketNetworkSnapshots` |
| 195 | `SetAddress` | `contract.address.rocketNetworkSnapshots` | `0x7603352f1c4752ac07aac94e48632b65fdf1d35c` | `0xe37f2d9dfb7397caf671df5190a5dfb601028f17` |
| 196 | `SetString` | `contract.abi.rocketNetworkSnapshots` | `eJzNVU1TgzAQ/SsO5x4qX4Xe9ObBS3W8OA6zCUtlig…` | `eJztVstOwzAQ/BXkc07hoao34MSBS0Fcqipy3G0bxd…` |
| 197 | `DeleteString` | `contract.name.old.rocketNetworkSnapshots` | `rocketNetworkSnapshots` | `` |
| 198 | `DeleteBool` | `contract.exists.old.rocketNetworkSnapshots` | `true` | `false` |
| 201 | `SetBool` | `contract.exists.rocketNetworkPenalties` | `false` | `true` |
| 202 | `SetString` | `contract.name.rocketNetworkPenalties` | `` | `rocketNetworkPenalties` |
| 203 | `SetAddress` | `contract.address.rocketNetworkPenalties` | `0x9294fc6f03c64cc217f5be8697ea3ed2de77e2f8` | `0xed0493de30e82be7c16c8925c7204ce9d1136b3a` |
| 204 | `SetString` | `contract.abi.rocketNetworkPenalties` | `eJzNVMtqwzAQ/JWis0+FluBb6amHQmjaUwhhLa+DqC…` | `eJzllctuwjAQRX+l8jqrqq0QO8SqCyQEtBuE0CSZIK…` |
| 205 | `DeleteString` | `contract.name.old.rocketNetworkPenalties` | `rocketNetworkPenalties` | `` |
| 206 | `DeleteBool` | `contract.exists.old.rocketNetworkPenalties` | `true` | `false` |
| 209 | `SetBool` | `contract.exists.rocketRewardsPool` | `false` | `true` |
| 210 | `SetString` | `contract.name.rocketRewardsPool` | `` | `rocketRewardsPool` |
| 211 | `SetAddress` | `contract.address.rocketRewardsPool` | `0xee4d2a71cf479e0d3d0c3c2c923dbfeb57e73111` | `0xcba5951fc706fc783b7c142dae8576ebe29c41fd` |
| 212 | `SetString` | `contract.abi.rocketRewardsPool` | `eJztWU1v4zYQ/SuFzj7oW3Ru3WyKBmiLIPEtCIwhOU…` | `eJztmV9v2jAQwL9KlWeeOm2qeFsrplXqJgSoe6gQMs…` |
| 213 | `DeleteString` | `contract.name.old.rocketRewardsPool` | `rocketRewardsPool` | `` |
| 214 | `DeleteBool` | `contract.exists.old.rocketRewardsPool` | `true` | `false` |
| 217 | `SetBool` | `contract.exists.rocketNodeDistributorDelegate` | `false` | `true` |
| 218 | `SetString` | `contract.name.rocketNodeDistributorDelegate` | `` | `rocketNodeDistributorDelegate` |
| 219 | `SetAddress` | `contract.address.rocketNodeDistributorDelegate` | `0x32778d6bf5b93b89177d328556eeeb35c09f472b` | `0x35a85d4c115801395e6e3abaa784fb05826f129d` |
| 220 | `SetString` | `contract.abi.rocketNodeDistributorDelegate` | `eJzNkktPwzAQhP+Lzzm4iZOa3iohbnABTlWF1vGmWE…` | `eJzNkcFOwzAMht8l556QQNNukxA3uACnqUJu40Gk1q…` |
| 221 | `DeleteString` | `contract.name.old.rocketNodeDistributorDelegate` | `rocketNodeDistributorDelegate` | `` |
| 222 | `DeleteBool` | `contract.exists.old.rocketNodeDistributorDelegate` | `true` | `false` |
| 225 | `SetBool` | `contract.exists.rocketClaimDAO` | `false` | `true` |
| 226 | `SetString` | `contract.name.rocketClaimDAO` | `` | `rocketClaimDAO` |
| 227 | `SetAddress` | `contract.address.rocketClaimDAO` | `0xfe6db0ce3f61a4ae04c0a3e62f775a6f511c9aac` | `0xfb2f2ab63dcf412ced6cde5f4f809215ed0c81aa` |
| 228 | `SetString` | `contract.abi.rocketClaimDAO` | `eJztWNtuGzcQ/ZVCz0bBy/KyfnOSPgRIGyNx0QfDEI…` | `eJztV0tr20AQ/itFZ9NDoSX45iQ9BNLGJC49GCPG0t…` |
| 229 | `DeleteString` | `contract.name.old.rocketClaimDAO` | `rocketClaimDAO` | `` |
| 230 | `DeleteBool` | `contract.exists.old.rocketClaimDAO` | `true` | `false` |
| 233 | `SetBool` | `contract.exists.rocketMinipoolBondReducer` | `false` | `true` |
| 234 | `SetString` | `contract.name.rocketMinipoolBondReducer` | `` | `rocketMinipoolBondReducer` |
| 235 | `SetAddress` | `contract.address.rocketMinipoolBondReducer` | `0xf7ab34c74c02407ed653ac9128731947187575c0` | `0xde8ab526b19fca2d5a57c4a78b698041717be591` |
| 236 | `SetString` | `contract.abi.rocketMinipoolBondReducer` | `eJzNlktP4zAQx7/KKuce7PgRmxsgrbTS7mrFIi4Iob…` | `eJzdlttqg0AQhl+l7LVXhZaQu7RQKLSlpCE3IYRRJ2…` |
| 237 | `DeleteString` | `contract.name.old.rocketMinipoolBondReducer` | `rocketMinipoolBondReducer` | `` |
| 238 | `DeleteBool` | `contract.exists.old.rocketMinipoolBondReducer` | `true` | `false` |
| 241 | `SetBool` | `contract.exists.rocketMinipoolManager` | `false` | `true` |
| 242 | `SetString` | `contract.name.rocketMinipoolManager` | `` | `rocketMinipoolManager` |
| 243 | `SetAddress` | `contract.address.rocketMinipoolManager` | `0xf82991bd8976c243eb3b7cddc52ab0fc8dc1246c` | `0xe54b8c641fd96de5d6747f47c19964c6b824d62c` |
| 244 | `SetString` | `contract.abi.rocketMinipoolManager` | `eJztWktv2zgQ/iuFzzmQEp+9Je0WWGC7CJpF91AUxZ…` | `eJztWl1v2jAU/StVnnmatGnqG7SrNGmdUJm6h6qqnO…` |
| 245 | `DeleteString` | `contract.name.old.rocketMinipoolManager` | `rocketMinipoolManager` | `` |
| 246 | `DeleteBool` | `contract.exists.old.rocketMinipoolManager` | `true` | `false` |
| 249 | `SetBool` | `contract.exists.rocketNetworkVoting` | `false` | `true` |
| 250 | `SetString` | `contract.name.rocketNetworkVoting` | `` | `rocketNetworkVoting` |
| 251 | `SetAddress` | `contract.address.rocketNetworkVoting` | `0x77cf0f32bdd06242465eb3318a81196194a13daa` | `0x994a9c49230fec0c127b8f42d6c5288f02610aed` |
| 252 | `SetString` | `contract.abi.rocketNetworkVoting` | `eJzNVstu2zAQ/JVCZx0sWtTDtyJGgRySFk3RHoIgWJ…` | `eJzVVMtugzAQ/JXKZ06pWkW5VeHSQx9qql6iKFpgQV…` |
| 253 | `DeleteString` | `contract.name.old.rocketNetworkVoting` | `rocketNetworkVoting` | `` |
| 254 | `DeleteBool` | `contract.exists.old.rocketNetworkVoting` | `true` | `false` |
| 257 | `SetBool` | `contract.exists.rocketMerkleDistributorMainnet` | `false` | `true` |
| 258 | `SetString` | `contract.name.rocketMerkleDistributorMainnet` | `` | `rocketMerkleDistributorMainnet` |
| 259 | `SetAddress` | `contract.address.rocketMerkleDistributorMainnet` | `0x5ce71e603b138f7e65029cc1918c0566ed0dbd4b` | `0xe4e2612ee8d7fdc8518faea85770a3b9c886e2f5` |
| 260 | `SetString` | `contract.abi.rocketMerkleDistributorMainnet` | `eJztVk2PmzAQ/SsVZw5AIJDcomqlrtSqq+yqPURRNN…` | `eJztVlFv2jAQ/iuVn3nqtKniDU2VVmnVEFTsAVXoSA…` |
| 261 | `DeleteString` | `contract.name.old.rocketMerkleDistributorMainnet` | `rocketMerkleDistributorMainnet` | `` |
| 262 | `DeleteBool` | `contract.exists.old.rocketMerkleDistributorMainnet` | `true` | `false` |
| 265 | `SetBool` | `contract.exists.rocketDAOProtocolSettingsProposals` | `false` | `true` |
| 266 | `SetString` | `contract.name.rocketDAOProtocolSettingsProposals` | `` | `rocketDAOProtocolSettingsProposals` |
| 267 | `SetAddress` | `contract.address.rocketDAOProtocolSettingsProposals` | `0x59cd103df1be2ebd80d45c54a3cde8d4f812c034` | `0xf6ad771dfb1cd10c66f688e251b5e5c21cbfdf81` |
| 268 | `SetString` | `contract.abi.rocketDAOProtocolSettingsProposals` | `eJzNlk1vm0AQhv9KxdkH2PDpW9z20EMkt0lziaJo2B…` | `eJzNll1PwjAUhv+K2fVuJNEQ7kC9MJEEBbkhCzmMw2…` |
| 269 | `DeleteString` | `contract.name.old.rocketDAOProtocolSettingsProposals` | `rocketDAOProtocolSettingsProposals` | `` |
| 270 | `DeleteBool` | `contract.exists.old.rocketDAOProtocolSettingsProposals` | `true` | `false` |
| 274 | `SetAddress` | `rewards.relay.address.0` | `0x5ce71e603b138f7e65029cc1918c0566ed0dbd4b` | `0xe4e2612ee8d7fdc8518faea85770a3b9c886e2f5` |
| 281 | `SetAddress` | `megapool.delegate.set.delegate.0` | `0x0000000000000000000000000000000000000000` | `0xca3dd4bee7c174903dbf66c3897c27e9adaaebdd` |
| 282 | `SetUint` | `megapool.delegate.set.meta` | `0` | `340282366920938463463374607431768211456` |
| 283 | `SetAddress` | `contract.address.rocketMegapoolDelegate` | `0x0000000000000000000000000000000000000000` | `0xca3dd4bee7c174903dbf66c3897c27e9adaaebdd` |
| 285 | `SetUint` | `dao.protocol.setting.megapool.megapool.time.before.dissolve` | `0` | `2419200` |
| 286 | `SetUint` | `dao.protocol.setting.megapool.megapool.dissolve.penalty` | `0` | `50000000000000000` |
| 287 | `SetUint` | `dao.protocol.setting.megapool.maximum.megapool.eth.penalty` | `0` | `612000000000000000000` |
| 288 | `SetUint` | `dao.protocol.setting.megapool.notify.threshold` | `0` | `112` |
| 289 | `SetUint` | `dao.protocol.setting.megapool.late.notify.fine` | `0` | `50000000000000000` |
| 290 | `SetUint` | `dao.protocol.setting.megapool.user.distribute.delay` | `0` | `1575` |
| 291 | `SetUint` | `dao.protocol.setting.megapool.user.distribute.delay.shortfall` | `0` | `6750` |
| 292 | `SetUint` | `dao.protocol.setting.megapool.megapool.penalty.threshold` | `0` | `510000000000000000` |
| 294 | `SetBool` | `dao.protocol.setting.megapool.deployed` | `false` | `true` |
| 295 | `SetBool` | `dao.security.allowed.setting.network.network.node.commission.share.security.council.adder` | `false` | `true` |
| 296 | `SetBool` | `dao.security.allowed.setting.network.network.submit.rewards.enabled` | `false` | `true` |
| 297 | `SetUint` | `dao.protocol.setting.deposit.deposit.assign.socialised.maximum` | `2` | `0` |
| 298 | `SetUint` | `dao.protocol.setting.deposit.express.queue.rate` | `0` | `4` |
| 299 | `SetUint` | `dao.protocol.setting.deposit.express.queue.tickets.base.provision` | `0` | `0` |
| 300 | `SetUint` | `dao.protocol.setting.network.network.node.commission.share` | `0` | `50000000000000000` |
| 301 | `SetUint` | `dao.protocol.setting.network.network.node.commission.share.security.council.adder` | `0` | `0` |
| 302 | `SetUint` | `dao.protocol.setting.network.network.voter.share` | `0` | `90000000000000000` |
| 303 | `SetUint` | `dao.protocol.setting.network.network.pdao.share` | `0` | `0` |
| 304 | `SetUint` | `dao.protocol.setting.network.network.max.node.commission.share.council.adder` | `0` | `10000000000000000` |
| 305 | `SetUint` | `dao.protocol.setting.network.network.max.reth.balance.delta` | `0` | `20000000000000000` |
| 306 | `SetUint` | `dao.protocol.setting.node.reduced.bond` | `0` | `4000000000000000000` |
| 307 | `SetUint` | `dao.protocol.setting.node.node.unstaking.period` | `0` | `2419200` |
| 308 | `SetUint` | `dao.protocol.setting.node.node.withdrawal.cooldown` | `0` | `0` |
| 309 | `SetUint` | `dao.protocol.setting.node.node.minimum.legacy.staked.rpl` | `0` | `150000000000000000` |
| 310 | `DeleteUint` | `dao.protocol.setting.node.node.per.minipool.stake.minimum` | `0` | `0` |
| 311 | `DeleteUint` | `dao.protocol.setting.node.node.per.minipool.stake.maximum` | `600000000000000000` | `0` |
| 312 | `SetBool` | `dao.protocol.setting.node.node.deposit.enabled` | `false` | `true` |
| 313 | `SetUint` | `dao.protocol.setting.minipool.minipool.maximum.penalty.count` | `0` | `2500` |
| 314 | `SetUint` | `dao.protocol.setting.security.upgrade.delay` | `0` | `604800` |
| 315 | `SetUint` | `dao.protocol.setting.security.upgradeveto.quorum` | `0` | `330000000000000000` |
| 316 | `SetUint` | `dao.protocol.setting.proposals.proposal.quorum` | `150000000000000000` | `150000000000000000` |
| 317 | `SetUint` | `dao.protocol.setting.proposals.proposal.veto.quorum` | `510000000000000000` | `200000000000000000` |
| 324 | `SetUint` | `network.revenue.node.share.value.1771372799` | `0` | `5000` |
| 330 | `SetUint` | `snapshot.time.length.network.revenue.node.share` | `0` | `1` |
| 331 | `SetBytes32` | `network.revenue.node.share.checkpoint.0` | `0x0000000000000000000000000000000000000000…` | `0x00000000699500ff000000000000000000000000…` |
| 332 | `SetUint` | `network.revenue.voter.share.value.1771372799` | `0` | `9000` |
| 338 | `SetUint` | `snapshot.time.length.network.revenue.voter.share` | `0` | `1` |
| 339 | `SetBytes32` | `network.revenue.voter.share.checkpoint.0` | `0x0000000000000000000000000000000000000000…` | `0x00000000699500ff000000000000000000000000…` |
| 340 | `SetUint` | `network.revenue.pdao.share.value.1771372799` | `0` | `0` |
| 346 | `SetUint` | `snapshot.time.length.network.revenue.pdao.share` | `0` | `1` |
| 347 | `SetBytes32` | `network.revenue.pdao.share.checkpoint.0` | `0x0000000000000000000000000000000000000000…` | `0x00000000699500ff000000000000000000000000…` |
| 348 | `SetString` | `protocol.version` | `1.3.1` | `1.4` |

**Effect counts:**

- `SetBool`: 39
- `SetString`: 71
- `SetAddress`: 38
- `DeleteString`: 24
- `DeleteBool`: 24
- `SetUint`: 33
- `DeleteUint`: 2
- `SetBytes32`: 3

## External Calls

| # | From | To | Selector | ETH | Success |
|---|---|---|---|---|---|
| 271 | `0x5b3b5c7639…` | `0xe4e2612ee8…` | `0x592e6f59` | 0 | ✅ |
| 275 | `0x5b3b5c7639…` | `0xd5bffeaa9f…` | `0x592e6f59` | 0 | ✅ |
| 284 | `0x5b3b5c7639…` | `0x40628faac2…` | `0x592e6f59` | 0 | ✅ |
| 318 | `0x5b3b5c7639…` | `0x9d9708da8e…` | `0xa3e99958` | 0 | ✅ |

## Trust Model and Observation Boundary

This attestation covers only typed RocketStorage mutations and declared external calls captured from the upgrade transaction replay. It does not cover state changes inside externally called contracts, events, or any other protocol invariants.

> **Disclaimer:** PASS means only that the replayed payload conforms to the reviewed manifest within the documented observation boundary. This is not an audit, security certificate, or proof that the upgrade is safe or correct in its entirety.

---
*Generated at: 2026-09-05T15:27:51.083874054+00:00*
