

// https://pen.iana.org/pen/PenApplication.page
// https://www.iana.org/assignments/enterprise-numbers/enterprise-numbers
// 
// 
// 1.3.6.1.4.1 - IANA-registered Private Enterprises
// https://www.alvestrand.no/objectid/1.3.6.1.4.1.html
// 

pub mod ibm {
    // 1.3.6.1.4.1.2 - IBM 
    
}

pub mod cisco {
    // 1.3.6.1.4.1.9 - Cisco 
    // NOTE: 主要目标为: 实现思科私有接口 `NetFlow-MIB` 。
    // 
    // NetFlow 设备支持情况 ( 社区问答，信息不全 ): 
    //     https://community.cisco.com/t5/security-documents/netflow-support-matrix/ta-p/3644638
    // NetFlow Analyzer - Supported Devices ( 第三方统计信息 ):
    //     https://www.manageengine.co.uk/products/netflow/supported-devices.html
    // NetFlow Support (维基百科):
    //     https://en.wikipedia.org/wiki/NetFlow#NetFlow_support
    // NetFlow Config:
    //     https://www.cisco.com/c/en/us/td/docs/ios/12_2/switch/configuration/guide/fswtch_c/xcfnfc.html
    // 
    
    // 产品: Catalyst 2960G
    // 型号: WS-C2960G-24TC-L
    // 链接: https://www.cisco.com/c/en/us/support/switches/catalyst-2960g-24tc-l-switch/model.html#~tab-downloads
    // NetFlow 支持情况:
    //      https://www.cisco.com/c/en/us/products/collateral/switches/catalyst-2960-x-series-switches/datasheet_c78-728232.html
    // 配置: https://www.cisco.com/c/en/us/td/docs/switches/lan/catalyst2960x/software/15-0_2_EX/security/configuration_guide/b_sec_152ex_2960-x_cg/b_sec_152ex_2960-x_cg_chapter_0100.html
}


pub mod hp {
    // 1.3.6.1.4.1.11 - Hewlett-Packard 
    
}


pub mod huawei {
    // 1.3.6.1.4.1.2011 - Huawei-3Com 
    // NOTE: 主要目标为: 实现华为私有接口 `HUAWEI-NETSTREAM-MIB`。
    // 
    // https://support.huawei.com/enterprise/en/doc/EDOC1000178181/e704d4b7/mib-example
    // https://github.com/yumaojun03/zabbix_monitor/tree/master/Huawei
    // 
    // HUAWEI-NETSTREAM-MIB
    //    http://support.huawei.com/enterprise/docinforeader!loadDocument1.action?contentId=DOC1000036830&partNo=10652
    //    https://github.com/openstack/compass-core/blob/master/mibs/HUAWEI-NETSTREAM-MIB.mib
    // 
    // 注: 华为设备当中，仅 S5720HI 支持 NetStream 功能。
    // 
    
    pub fn hwNetStreamlastchangedtime() {
        unimplemented!()
    }

    pub fn hwNetStream16BitIndex() {
        unimplemented!()
    }

    pub fn hwifNet32BitIndex() {
        unimplemented!()
    }
}

pub mod h3c {
    // 1.3.6.1.4.1.25506 - H3C
    
}

pub mod zhongxing {
    // 1.3.6.1.4.1.3902 - Zhongxing Telecom Co.,ltd. (abbr. ZTE)
    
}

