use crate::contact::Contact;

pub struct Member(Contact);

pub struct NormalMember(Member);

impl TryFrom<Member> for NormalMember {
    type Error = Member;

    fn try_from(member: Member) -> Result<Self, Self::Error> {
        Ok(Self(member))
    }
}

impl From<NormalMember> for Member {
    fn from(nm: NormalMember) -> Self {
        nm.0
    }
}


pub struct AnonymousMember(Member);


impl TryFrom<Member> for AnonymousMember {
    type Error = Member;

    fn try_from(member: Member) -> Result<Self, Self::Error> {
        Ok(Self(member))
    }
}

impl From<AnonymousMember> for Member {
    fn from(am: AnonymousMember) -> Self {
        am.0
    }
}