create table public."user"
(
    id         uuid                     default gen_random_uuid() not null
        primary key,
    name       varchar                                            not null,
    password   varchar                                            not null,
    created_at timestamp with time zone default now()             not null,
    deleted    boolean                  default false             not null,
    email      varchar                                            not null
);

create unique index user_email_index
    on public."user" (email);

create table public.admin
(
    id         uuid                     default gen_random_uuid() not null
        primary key,
    name       varchar                                            not null,
    password   varchar                                            not null,
    created_at timestamp with time zone default now()             not null,
    deleted    boolean                  default false             not null,
    email      varchar                                            not null
);

create table public.flow
(
    id         uuid                     default gen_random_uuid() not null
        primary key,
    name       varchar                                            not null,
    user_id    uuid                                               not null
        constraint flow_user_id_fk
            references public."user",
    created_at timestamp with time zone default now()             not null
);

create unique index flow_user_name_uindex
    on public.flow (name);

create table public.step_send_media_message
(
    id      uuid default gen_random_uuid() not null
        primary key,
    url     varchar                        not null,
    header  varchar,
    content varchar,
    footer  varchar
);

create table public.flow_editor
(
    id   uuid  not null
        primary key
        references public.flow,
    data jsonb not null
);

create table public.step
(
    id         uuid                     default gen_random_uuid() not null
        primary key,
    t          varchar                                            not null,
    flow_id    uuid                                               not null
        references public.flow,
    created_at timestamp with time zone default now()             not null
);

create table public.step_send_message
(
    id      uuid default gen_random_uuid() not null
        constraint step_send_text_message_pkey
            primary key
        references public.step,
    content varchar                        not null
);

create table public.flow_connection
(
    id   uuid not null
        primary key,
    "to" uuid not null
        references public.step
);

create table public.step_wait_for_reply
(
    id uuid default gen_random_uuid() not null
        primary key
        references public.step
);

create table public.step_wait_for_reply_branch
(
    id         uuid                     default gen_random_uuid() not null
        primary key,
    text       varchar                                            not null,
    contains   boolean                  default true              not null,
    fuzzy      boolean                  default true              not null,
    parent_id  uuid                                               not null
        references public.step_wait_for_reply,
    smart      boolean                  default false             not null,
    created_at timestamp with time zone default now()             not null
);

create table public.webhook
(
    id         uuid                     default gen_random_uuid() not null
        primary key,
    t          varchar                                            not null,
    data       jsonb                                              not null,
    created_at timestamp with time zone default now()             not null
);

create table public.channel_instagram
(
    id    varchar not null
        primary key
        references public.channel_instagram,
    name  varchar not null,
    token varchar not null
);

create unique index channel_ig_pk
    on public.channel_instagram (id);

create table public.channel
(
    id         varchar                                not null
        primary key,
    created_at timestamp with time zone default now() not null,
    owner_id   uuid                                   not null
        references public."user",
    name       varchar                                not null,
    t          varchar                                not null
);

create table public.channel_whatsapp
(
    id              varchar                                not null
        constraint channel_wa_pkey
            primary key
        references public.channel,
    created_at      timestamp with time zone default now() not null,
    host            varchar                                not null,
    name            varchar                                not null,
    token           varchar                                not null,
    token_expire_at timestamp with time zone               not null,
    password        varchar                                not null,
    state           varchar                                not null
);

create table public.channel_facebook
(
    id    varchar not null
        primary key
        references public.channel,
    name  varchar not null,
    token varchar not null
);

create unique index channel_fb_pk
    on public.channel_facebook (id);

create table public.contact
(
    name       varchar                                            not null,
    ext_id     varchar                                            not null,
    channel_id varchar                                            not null,
    created_at timestamp with time zone default now()             not null,
    id         uuid                     default gen_random_uuid() not null
        primary key
);

create unique index contact_channel_uindex
    on public.contact (channel_id, ext_id);

create unique index id
    on public.contact (id);

create table public.message
(
    id         uuid                     default gen_random_uuid()         not null
        primary key,
    status     varchar(255)             default 'send'::character varying not null,
    created_at timestamp with time zone default now()                     not null,
    created_by varchar(255)                                               not null,
    reply_for  varchar(255),
    mid        varchar                                                    not null,
    contact_id uuid                                                       not null
        constraint message_contact_id_fk
            references public.contact,
    t          varchar                                                    not null
);

create table public.contact_step
(
    id              uuid                     default gen_random_uuid() not null
        primary key,
    step_id         uuid
        references public.step,
    created_at      timestamp with time zone default now()             not null,
    processed_at    timestamp with time zone,
    log             varchar,
    next_id         uuid
        references public.step,
    meta            json,
    conversation_id uuid                     default gen_random_uuid() not null,
    contact_id      uuid                                               not null
        constraint contact_step_contact_id_fk
            references public.contact
);

create table public.conversation_variable
(
    id              uuid                     default gen_random_uuid()           not null
        constraint conversation_variable_pk
            primary key,
    name            varchar                                                      not null,
    value           varchar                                                      not null,
    number          integer,
    vartype         varchar                  default 'string'::character varying not null,
    created_at      timestamp with time zone default now()                       not null,
    contact_step_id uuid                                                         not null
        constraint conversation_variable_contact_step_id_fk
            references public.contact_step,
    constraint conversation_variable_name_con_upk
        unique (contact_step_id, name)
);

create table public.message_media
(
    id         uuid                                       not null
        constraint message_text_pkey
            primary key
        constraint message_text_msg_id_fk
            references public.message,
    url        varchar                                    not null,
    media_type varchar default 'image'::character varying not null,
    text       varchar
);

create table public.message_text
(
    id   uuid    not null
        constraint message_text_pkey1
            primary key
        references public.message,
    text varchar not null
);

