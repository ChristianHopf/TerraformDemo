variable "env" {
    type = string
}

variable "project" {
    type = string
}

variable "workspace" {
    type = string
}

variable "labels" {
    type = list(object({ label = string, value = string }))
    description = "labels that will be applied to all docker containers"
    default = []
}