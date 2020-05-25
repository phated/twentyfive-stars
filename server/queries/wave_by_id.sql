SELECT nodes.node_id, tcg_id, name, released
FROM waves, nodes
WHERE waves.id = nodes.id AND nodes.id = $1;
