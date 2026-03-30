use crate::pdu::message_id::MessageId;

const BASE_LENGTH: usize = 126;
const MAX_LENGTH: usize = BASE_LENGTH + 21 * 100 + 160;

#[derive(Debug, Default)]
pub struct SubmitRequestPayload {
    /**
     * 信息标识，由SP侧短信网关本身产生，本处填空
     */
    pub msg_id: MessageId,

    /**
     * 相同Msg_Id的信息总条数，从1开始
     */
    pub pk_total: u8,

    /**
     * 相同Msg_Id的信息序号，从1开始
     */
    pub pk_number: u8,

    /**
     * 是否要求返回状态确认报告：
     * 0：不需要
     * 1：需要
     * 2：产生SMC话单
     */
    pub registered_delivery: u8,

    /**
     * 信息级别
     */
    pub msg_level: u8,

    /**
     * 业务类型，是数字、字母和符号的组合。
     */
    pub service_id: String,

    /**
     * 计费用户类型字段
     * 0：对目的终端MSISDN计费；
     * 1：对源终端MSISDN计费；
     * 2：对SP计费;
     * 3：表示本字段无效，对谁计费参见Fee_terminal_Id字段。
     */
    pub fee_user_type: u8,

    /**
     * 被计费用户的号码（如本字节填空，则表示本字段无效，对谁计费参见Fee_UserType字段，本字段与Fee_UserType字段互斥）
     */
    pub fee_terminal_id: String,

    /**
     * GSM协议类型，详见GSM03.40中的9.2.3.9
     */
    pub tp_pid: u8,

    /**
     * GSM协议类型，详见GSM03.40中的9.2.3.23
     * 仅使用1位，右对齐
     */
    pub tp_udhi: u8,

    /**
     * 信息格式
     * 0：ASCII串
     * 3：短信写卡操作
     * 4：二进制信息
     * 8：UCS2编码
     * 15：含GB汉字
     */
    pub msg_fmt: u8,

    /**
     * 信息内容来源(SP_Id)
     */
    pub msg_src: String,

    /**
     * 资费类别
     * 01：对“计费用户号码”免费
     * 02：对“计费用户号码”按条计信息费
     * 03：对“计费用户号码”按包月收取信息费
     * 04：对“计费用户号码”的信息费封顶
     * 05：对“计费用户号码”的收费是由SP实现
     */
    pub fee_type: String,

    /**
     * 资费代码（以分为单位）
     */
    pub fee_code: String,

    /**
     * 存活有效期，格式遵循SMPP3.3协议
     */
    pub valid_time: String,

    /**
     * 定时发送时间，格式遵循SMPP3.3协议
     */
    pub at_time: String,

    /**
     * 源号码
     * SP的服务代码或前缀为服务代码的长号码,
     * 网关将该号码完整的填到SMPP协议Submit_SM消息相应的source_addr字段，
     * 该号码最终在用户手机上显示为短消息的主叫号码
     */
    pub src_id: String,

    /**
     * 接收信息的用户数量（<=100）
     */
    pub dest_usr_tl: u8,

    /**
     * 接收业务的MSISDN号码
     */
    pub dest_terminal_id: Vec<String>,

    /**
     * 信息长度
     *
     * 当 msg_fmt == 0 时，msg_length < 160
     * 其他情况，msg_length <= 140
     */
    pub msg_length: u8,

    /**
     * 信息内容
     */
    pub msg_content: Vec<u8>,

    /**
     * 保留
     */
    pub reserve: [u8; 8],
}

impl SubmitRequestPayload {
    pub fn len(&self) -> usize {
        BASE_LENGTH + (21 * self.dest_usr_tl + self.msg_length) as usize
    }
}
