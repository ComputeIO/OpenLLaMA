type GgmlBackendSchedEvalCallback = fn(struct ggml_tensor * t, bool ask, void * user_data) -> bool;