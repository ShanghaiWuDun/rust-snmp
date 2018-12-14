SNMP 协议的 Rust 实现
=======================

:Date: 2018/12/05

.. contents::



介绍
---------

TODO.


SNMP 协议版本支持情况
----------------------

*   [开发中] SNMP v1
*   [开发中] SNMP v2c
*   [无计划] SNMP v2p
*   [无计划] SNMP v2u
*   [无计划] SNMP v3


运行
----------

.. code:: bash
    
    cargo build
    cargo run



参考
---------

*   `Simple Network Management Protocol <http://docwiki.cisco.com/wiki/Simple_Network_Management_Protocol>`_
*   `SNMPv1 Message Format <http://www.tcpipguide.com/free/t_SNMPVersion1SNMPv1MessageFormat.htm>`_
*   `SNMPv2c Message Format <http://www.tcpipguide.com/free/t_SNMPVersion2SNMPv2MessageFormats-3.htm>`_
*   `SNMPv2p Message Format <http://www.tcpipguide.com/free/t_SNMPVersion2SNMPv2MessageFormats-2.htm>`_ , 实际上并没有具体应用支持该协议。
*   `SNMPv2u Message Format <http://www.tcpipguide.com/free/t_SNMPVersion2SNMPv2MessageFormats-4.htm>`_ , 实际上并没有具体应用支持该协议。
*   `SNMPv3 Message Format <http://www.tcpipguide.com/free/t_SNMPVersion3SNMPv3MessageFormat.htm>`_

*   `ASN.1 & BER & DER <http://luca.ntop.org/Teaching/Appunti/asn1.html>`_
*   `ASN.1 DER Encoding <http://jianiau.blogspot.com/2014/06/asn1-der-encoding-decoding.html>`_


*已废弃规范:*

*   `RFC 1065 <https://tools.ietf.org/html/rfc1065>`_
*   `RFC 1066 <https://tools.ietf.org/html/rfc1066>`_
*   `RFC 1067 <https://tools.ietf.org/html/rfc1067>`_

*SNMP V1:*

*   `RFC 1155 <https://tools.ietf.org/html/rfc1155>`_
*   `RFC 1156 <https://tools.ietf.org/html/rfc1156>`_
*   `RFC 1157 <https://tools.ietf.org/html/rfc1157>`_



