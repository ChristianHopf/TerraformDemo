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

variable "admin_email" {
  type = string
  default = "cehopf@gmail.com"
}

variable "https" {
  type = number
  default = 1
}

variable "hosted_zones" {
  type = map(object({
    name = string
    hosts = list(string)
    local_port = number
  }))
}
