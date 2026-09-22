resource "neon_project" "this" {
  history_retention_seconds = 21600
  name                      = var.project
  region_id                 = var.region
}

resource "neon_role" "this" {
  branch_id  = neon_project.this.default_branch_id
  name       = "admin"
  project_id = neon_project.this.id
}

resource "neon_database" "this" {
  branch_id  = neon_project.this.default_branch_id
  name       = var.project
  owner_name = neon_role.this.name
  project_id = neon_project.this.id
}

data "neon_project" "this" {
  id = neon_project.this.id
}
