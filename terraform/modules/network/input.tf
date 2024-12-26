variable "env" {
    type = string
}

variable "postfix" {
    type = string
}

variable "project" {
    type = string
}

variable "workspace" {
    type = string
}

variable "labels" {
    type = list(object({label = string, value = string }))
    description = "labels applied to all docker containers"
    default = []
}

variable "log_opts" {
    type = map(any)
    description = "options for docker logging"
    default = { "max-size" = "10m", "max-file" = "3" }
}