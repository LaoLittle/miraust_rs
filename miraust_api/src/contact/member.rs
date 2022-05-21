use crate::contact::Contact;

pub struct Member {
    pub(crate) inner: Contact,
}

pub struct NormalMember {
    pub(crate) member: Member,
}

impl TryFrom<Member> for NormalMember {
    type Error = Member;

    fn try_from(member: Member) -> Result<Self, Self::Error> {
        Ok(Self { member })
    }
}

impl From<NormalMember> for Member {
    fn from(nm: NormalMember) -> Self {
        nm.member
    }
}


pub struct AnonymousMember {
    pub(crate) member: Member,
}


impl TryFrom<Member> for AnonymousMember {
    type Error = Member;

    fn try_from(member: Member) -> Result<Self, Self::Error> {
        Ok(Self { member })
    }
}

impl From<AnonymousMember> for Member {
    fn from(am: AnonymousMember) -> Self {
        am.member
    }
}