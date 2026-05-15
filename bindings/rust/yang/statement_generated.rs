use strum_macros::{Display, EnumCount, EnumIter, EnumString};

#[derive(Clone, Copy, Debug, Display, EnumCount, EnumIter, EnumString, Eq, Hash, PartialEq)]
pub enum StatementKind {
    #[strum(to_string = "action", serialize = "action_stmt")]
    Action,
    #[strum(to_string = "anydata", serialize = "anydata_stmt")]
    Anydata,
    #[strum(to_string = "anyxml", serialize = "anyxml_stmt")]
    Anyxml,
    #[strum(to_string = "argument", serialize = "argument_stmt")]
    Argument,
    #[strum(to_string = "augment", serialize = "augment_stmt")]
    Augment,
    #[strum(to_string = "base", serialize = "base_stmt")]
    Base,
    #[strum(to_string = "belongs-to", serialize = "belongs_to_stmt")]
    BelongsTo,
    #[strum(to_string = "bit", serialize = "bit_stmt")]
    Bit,
    #[strum(to_string = "case", serialize = "case_stmt")]
    Case,
    #[strum(to_string = "choice", serialize = "choice_stmt")]
    Choice,
    #[strum(to_string = "config", serialize = "config_stmt")]
    Config,
    #[strum(to_string = "contact", serialize = "contact_stmt")]
    Contact,
    #[strum(to_string = "container", serialize = "container_stmt")]
    Container,
    #[strum(to_string = "default", serialize = "default_stmt")]
    Default,
    #[strum(to_string = "description", serialize = "description_stmt")]
    Description,
    #[strum(to_string = "deviate-add", serialize = "deviate_add_stmt")]
    DeviateAdd,
    #[strum(to_string = "deviate-delete", serialize = "deviate_delete_stmt")]
    DeviateDelete,
    #[strum(to_string = "deviate-not-supported", serialize = "deviate_not_supported_stmt")]
    DeviateNotSupported,
    #[strum(to_string = "deviate-replace", serialize = "deviate_replace_stmt")]
    DeviateReplace,
    #[strum(to_string = "deviation", serialize = "deviation_stmt")]
    Deviation,
    #[strum(to_string = "enum", serialize = "enum_stmt")]
    Enum,
    #[strum(to_string = "error-app-tag", serialize = "error_app_tag_stmt")]
    ErrorAppTag,
    #[strum(to_string = "error-message", serialize = "error_message_stmt")]
    ErrorMessage,
    #[strum(to_string = "extension", serialize = "extension_stmt")]
    Extension,
    #[strum(to_string = "feature", serialize = "feature_stmt")]
    Feature,
    #[strum(to_string = "fraction-digits", serialize = "fraction_digits_stmt")]
    FractionDigits,
    #[strum(to_string = "grouping", serialize = "grouping_stmt")]
    Grouping,
    #[strum(to_string = "identity", serialize = "identity_stmt")]
    Identity,
    #[strum(to_string = "if-feature", serialize = "if_feature_stmt")]
    IfFeature,
    #[strum(to_string = "import", serialize = "import_stmt")]
    Import,
    #[strum(to_string = "include", serialize = "include_stmt")]
    Include,
    #[strum(to_string = "input", serialize = "input_stmt")]
    Input,
    #[strum(to_string = "key", serialize = "key_stmt")]
    Key,
    #[strum(to_string = "leaf-list", serialize = "leaf_list_stmt")]
    LeafList,
    #[strum(to_string = "leaf", serialize = "leaf_stmt")]
    Leaf,
    #[strum(to_string = "length", serialize = "length_stmt")]
    Length,
    #[strum(to_string = "list", serialize = "list_stmt")]
    List,
    #[strum(to_string = "mandatory", serialize = "mandatory_stmt")]
    Mandatory,
    #[strum(to_string = "max-elements", serialize = "max_elements_stmt")]
    MaxElements,
    #[strum(to_string = "min-elements", serialize = "min_elements_stmt")]
    MinElements,
    #[strum(to_string = "modifier", serialize = "modifier_stmt")]
    Modifier,
    #[strum(to_string = "module", serialize = "module_stmt")]
    Module,
    #[strum(to_string = "must", serialize = "must_stmt")]
    Must,
    #[strum(to_string = "namespace", serialize = "namespace_stmt")]
    Namespace,
    #[strum(to_string = "notification", serialize = "notification_stmt")]
    Notification,
    #[strum(to_string = "ordered-by", serialize = "ordered_by_stmt")]
    OrderedBy,
    #[strum(to_string = "organization", serialize = "organization_stmt")]
    Organization,
    #[strum(to_string = "output", serialize = "output_stmt")]
    Output,
    #[strum(to_string = "path", serialize = "path_stmt")]
    Path,
    #[strum(to_string = "pattern", serialize = "pattern_stmt")]
    Pattern,
    #[strum(to_string = "position", serialize = "position_stmt")]
    Position,
    #[strum(to_string = "prefix", serialize = "prefix_stmt")]
    Prefix,
    #[strum(to_string = "presence", serialize = "presence_stmt")]
    Presence,
    #[strum(to_string = "range", serialize = "range_stmt")]
    Range,
    #[strum(to_string = "reference", serialize = "reference_stmt")]
    Reference,
    #[strum(to_string = "refine", serialize = "refine_stmt")]
    Refine,
    #[strum(to_string = "require-instance", serialize = "require_instance_stmt")]
    RequireInstance,
    #[strum(to_string = "revision-date", serialize = "revision_date_stmt")]
    RevisionDate,
    #[strum(to_string = "revision", serialize = "revision_stmt")]
    Revision,
    #[strum(to_string = "rpc", serialize = "rpc_stmt")]
    Rpc,
    #[strum(to_string = "status", serialize = "status_stmt")]
    Status,
    #[strum(to_string = "submodule", serialize = "submodule_stmt")]
    Submodule,
    #[strum(to_string = "type", serialize = "type_stmt")]
    Type,
    #[strum(to_string = "typedef", serialize = "typedef_stmt")]
    Typedef,
    #[strum(to_string = "unique", serialize = "unique_stmt")]
    Unique,
    #[strum(to_string = "units", serialize = "units_stmt")]
    Units,
    #[strum(to_string = "unknown", serialize = "unknown_stmt")]
    Unknown,
    #[strum(to_string = "uses-augment", serialize = "uses_augment_stmt")]
    UsesAugment,
    #[strum(to_string = "uses", serialize = "uses_stmt")]
    Uses,
    #[strum(to_string = "value", serialize = "value_stmt")]
    Value,
    #[strum(to_string = "when", serialize = "when_stmt")]
    When,
    #[strum(to_string = "yang-version", serialize = "yang_version_stmt")]
    YangVersion,
    #[strum(to_string = "yin-element", serialize = "yin_element_stmt")]
    YinElement,
}