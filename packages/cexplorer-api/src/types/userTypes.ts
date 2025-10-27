export type UserProfile = {
  name: string;
  picture: string;
  social: {
    web: string;
    xcom: string;
    telegram: string;
    discord: string;
    patreon: string;
    facebook: string;
    instagram: string;
    github: string;
    linkedin: string;
  };
};

export interface User {
  profile: UserProfile;
  address: string;
  membership: {
    og: number;
    nfts: number;
    extra: string[];
  };
}
