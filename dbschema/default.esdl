module default {
  type subscriptions {
    required property email -> str { constraint exclusive };
    required property name -> str;
    required property subscrid_at -> datetime;
  }
}
