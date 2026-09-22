output "database_url" {
  sensitive = true
  value     = "postgresql://${neon_role.this.name}:${neon_role.this.password}@${data.neon_project.this.database_host_pooler}/${neon_database.this.name}?sslmode=require"
}
output "database_url_direct" {
  sensitive = true
  value     = "postgresql://${neon_role.this.name}:${neon_role.this.password}@${data.neon_project.this.database_host}/${neon_database.this.name}?sslmode=require"
}
