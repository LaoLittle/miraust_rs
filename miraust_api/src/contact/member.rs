use crate::contact::Contact;
use crate::jni_struct::GlobalRef;

pub struct Member {
    pub(crate) id: i64,
    pub(crate) inner: GlobalRef,
}

impl Contact for Member {
    fn id(&self) -> u64 {
        self.id as _
    }

    fn send_message(&self) {
        todo!()
    }
}

pub struct NormalMember {
    pub(crate) member: Member
}

impl TryFrom<Member> for NormalMember {
    type Error = Member;

    fn try_from(member: Member) -> Result<Self, Self::Error> {
        Ok(Self { member })
    }
}

impl Into<Member> for NormalMember {
    fn into(self) -> Member {
        self.member
    }
}

impl Contact for NormalMember {
    fn id(&self) -> u64 {
        self.member.id()
    }

    fn send_message(&self) {
        self.member.send_message()
    }
}

pub struct AnonymousMember {
    pub(crate) member: Member
}

impl Contact for AnonymousMember {
    fn id(&self) -> u64 {
        self.member.id()
    }

    fn send_message(&self) {
        self.member.send_message()
    }
}

impl TryFrom<Member> for AnonymousMember {
    type Error = Member;

    fn try_from(member: Member) -> Result<Self, Self::Error> {
        Ok(Self { member })
    }
}

impl Into<Member> for AnonymousMember {
    fn into(self) -> Member {
        self.member
    }
}