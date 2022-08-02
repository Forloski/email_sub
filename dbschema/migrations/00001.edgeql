CREATE MIGRATION m1l2d3qwzro76r7akicfn3craxhnl3te2qamgnf2jqhkvt555xt6aa
    ONTO initial
{
  CREATE TYPE default::subscriptions {
      CREATE REQUIRED PROPERTY email -> std::str {
          CREATE CONSTRAINT std::exclusive;
      };
      CREATE REQUIRED PROPERTY name -> std::str;
      CREATE REQUIRED PROPERTY subscrid_at -> std::datetime;
  };
};
