<script setup lang="ts">
import { useQueryStore } from '../../stores/query'

const queryStore = useQueryStore()

function generateSelectSql() {
  if (!queryStore.selectedTable) return

  const { schema, table_name, columns } = queryStore.selectedTable
  const columnNames = columns.map(c => `"${c.name}"`).join(', ')
  const sql = `SELECT ${columnNames}\nFROM "${schema}"."${table_name}"\nLIMIT 100;`

  queryStore.setCurrentSql(sql)
}

function closeDetail() {
  queryStore.closeTableDetail()
}
</script>

<template>
  <div class="table-detail-card">
    <div class="detail-header">
      <div class="header-title">
        <h3 class="table-name">
          <span class="schema-name">{{ queryStore.selectedTable?.schema }}</span>
          <span class="separator">.</span>
          <span class="name">{{ queryStore.selectedTable?.table_name }}</span>
        </h3>
      </div>
      <div class="header-actions">
        <button class="btn btn-primary" @click="generateSelectSql">
          生成SELECT语句
        </button>
        <button class="btn btn-close" @click="closeDetail" title="关闭">
          ✕
        </button>
      </div>
    </div>

    <div class="detail-content">
      <div class="section">
        <h4 class="section-title">列信息</h4>
        <div class="table-container">
          <table class="columns-table">
            <thead>
              <tr>
                <th class="col-name">列名</th>
                <th class="col-type">数据类型</th>
                <th class="col-nullable">可空</th>
                <th class="col-default">默认值</th>
                <th class="col-pk">主键</th>
              </tr>
            </thead>
            <tbody>
              <tr
                v-for="column in queryStore.selectedTable?.columns"
                :key="column.name"
                :class="{ 'is-pk': column.is_primary_key }"
              >
                <td class="col-name">
                  <span class="column-name">{{ column.name }}</span>
                </td>
                <td class="col-type">
                  <span class="data-type">{{ column.data_type }}</span>
                </td>
                <td class="col-nullable">
                  <span class="nullable-badge" :class="{ 'is-nullable': column.is_nullable }">
                    {{ column.is_nullable ? '是' : '否' }}
                  </span>
                </td>
                <td class="col-default">
                  <span class="default-value" :title="column.column_default">
                    {{ column.column_default || '-' }}
                  </span>
                </td>
                <td class="col-pk">
                  <span v-if="column.is_primary_key" class="pk-badge" title="主键">
                    🔑
                  </span>
                  <span v-else>-</span>
                </td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>

      <div class="section">
        <h4 class="section-title">统计信息</h4>
        <div class="stats-grid">
          <div class="stat-item">
            <span class="stat-label">总列数</span>
            <span class="stat-value">{{ queryStore.selectedTable?.columns.length || 0 }}</span>
          </div>
          <div class="stat-item">
            <span class="stat-label">主键列</span>
            <span class="stat-value">
              {{ queryStore.selectedTable?.columns.filter(c => c.is_primary_key).length || 0 }}
            </span>
          </div>
          <div class="stat-item">
            <span class="stat-label">可空列</span>
            <span class="stat-value">
              {{ queryStore.selectedTable?.columns.filter(c => c.is_nullable).length || 0 }}
            </span>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.table-detail-card {
  background: white;
  border-radius: 8px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  width: 100%;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.detail-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 20px 24px;
  border-bottom: 1px solid #eee;
  background-color: #fafafa;
}

.header-title {
  flex: 1;
  min-width: 0;
}

.table-name {
  margin: 0;
  font-size: 18px;
  font-weight: 600;
  color: #333;
  display: flex;
  align-items: center;
  gap: 4px;
  flex-wrap: wrap;
}

.schema-name {
  color: #666;
  font-weight: 500;
}

.separator {
  color: #999;
}

.name {
  color: #333;
  font-family: 'Consolas', 'Monaco', 'Courier New', monospace;
}

.header-actions {
  display: flex;
  align-items: center;
  gap: 12px;
}

.btn {
  padding: 8px 16px;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 14px;
  transition: all 0.2s;
}

.btn-primary {
  background-color: #2196F3;
  color: white;
}

.btn-primary:hover {
  background-color: #1976D2;
}

.btn-close {
  width: 32px;
  height: 32px;
  padding: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  background-color: transparent;
  color: #999;
  font-size: 18px;
}

.btn-close:hover {
  background-color: #f5f5f5;
  color: #333;
}

.detail-content {
  flex: 1;
  overflow-y: auto;
  padding: 24px;
}

.section {
  margin-bottom: 24px;
}

.section-title {
  font-size: 14px;
  font-weight: 600;
  color: #666;
  margin: 0 0 12px 0;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.table-container {
  border: 1px solid #e0e0e0;
  border-radius: 6px;
  overflow: hidden;
}

.columns-table {
  width: 100%;
  border-collapse: collapse;
  font-size: 13px;
}

.columns-table th {
  background-color: #f5f5f5;
  padding: 12px 16px;
  text-align: left;
  font-weight: 600;
  color: #555;
  border-bottom: 1px solid #e0e0e0;
}

.columns-table td {
  padding: 12px 16px;
  border-bottom: 1px solid #f0f0f0;
}

.columns-table tr:last-child td {
  border-bottom: none;
}

.columns-table tr:hover {
  background-color: #fafafa;
}

.columns-table tr.is-pk {
  background-color: #fff8e1;
}

.columns-table tr.is-pk:hover {
  background-color: #ffecb3;
}

.col-name {
  width: 25%;
}

.col-type {
  width: 20%;
}

.col-nullable {
  width: 10%;
  text-align: center;
}

.col-default {
  width: 30%;
}

.col-pk {
  width: 10%;
  text-align: center;
}

.column-name {
  font-family: 'Consolas', 'Monaco', 'Courier New', monospace;
  font-weight: 500;
  color: #333;
}

.data-type {
  color: #666;
  font-family: 'Consolas', 'Monaco', 'Courier New', monospace;
}

.nullable-badge {
  display: inline-block;
  padding: 2px 8px;
  border-radius: 12px;
  font-size: 11px;
  background-color: #e0e0e0;
  color: #666;
}

.nullable-badge.is-nullable {
  background-color: #e3f2fd;
  color: #1976D2;
}

.default-value {
  color: #888;
  font-family: 'Consolas', 'Monaco', 'Courier New', monospace;
  font-size: 12px;
  max-width: 200px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  display: inline-block;
}

.pk-badge {
  font-size: 14px;
}

.stats-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(120px, 1fr));
  gap: 16px;
}

.stat-item {
  background-color: #f5f5f5;
  padding: 16px;
  border-radius: 6px;
  text-align: center;
}

.stat-label {
  display: block;
  font-size: 12px;
  color: #666;
  margin-bottom: 4px;
}

.stat-value {
  display: block;
  font-size: 24px;
  font-weight: 600;
  color: #333;
}

@media (max-width: 640px) {
  .table-detail-panel {
    max-height: 90vh;
    margin: 10px;
  }

  .detail-header {
    flex-direction: column;
    align-items: flex-start;
    gap: 12px;
  }

  .header-actions {
    width: 100%;
    justify-content: space-between;
  }

  .columns-table {
    font-size: 12px;
  }

  .columns-table th,
  .columns-table td {
    padding: 8px 10px;
  }

  .col-default {
    max-width: 100px;
  }
}
</style>
