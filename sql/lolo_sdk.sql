SET NAMES utf8mb4;
SET FOREIGN_KEY_CHECKS = 0;

-- ----------------------------
-- Table structure for of_quick
-- ----------------------------
DROP TABLE IF EXISTS `of_quick`;
CREATE TABLE `of_quick`  (
  `id` int UNSIGNED NOT NULL AUTO_INCREMENT,
  `username` varchar(191) CHARACTER SET utf8mb4 COLLATE utf8mb4_bin NULL DEFAULT NULL,
  `password` longtext CHARACTER SET utf8mb4 COLLATE utf8mb4_bin NOT NULL,
  `reg_device` longtext CHARACTER SET utf8mb4 COLLATE utf8mb4_bin NOT NULL,
  `user_token` longtext CHARACTER SET utf8mb4 COLLATE utf8mb4_bin NULL,
  `auth_token` longtext CHARACTER SET utf8mb4 COLLATE utf8mb4_bin NOT NULL,
  PRIMARY KEY (`id`) USING BTREE,
  UNIQUE INDEX `uni_of_quick_username`(`username` ASC) USING BTREE,
  INDEX `idx_of_quick_id`(`id` ASC) USING BTREE,
  INDEX `idx_of_quick_username`(`username` ASC) USING BTREE
) ENGINE = InnoDB AUTO_INCREMENT = 1 CHARACTER SET = utf8mb4 COLLATE = utf8mb4_bin ROW_FORMAT = Dynamic;

-- ----------------------------
-- Table structure for of_quick_check
-- ----------------------------
DROP TABLE IF EXISTS `of_quick_check`;
CREATE TABLE `of_quick_check`  (
  `uid` varchar(191) CHARACTER SET utf8mb4 COLLATE utf8mb4_bin NOT NULL,
  `gate_token` longtext CHARACTER SET utf8mb4 COLLATE utf8mb4_bin NULL,
  `last_package_name` longtext CHARACTER SET utf8mb4 COLLATE utf8mb4_bin NULL,
  `gen_key` longtext CHARACTER SET utf8mb4 COLLATE utf8mb4_bin NULL,
  PRIMARY KEY (`uid`) USING BTREE,
  UNIQUE INDEX `uni_of_quick_check_uid`(`uid` ASC) USING BTREE,
  INDEX `idx_of_quick_check_uid`(`uid` ASC) USING BTREE
) ENGINE = InnoDB CHARACTER SET = utf8mb4 COLLATE = utf8mb4_bin ROW_FORMAT = Dynamic;

SET FOREIGN_KEY_CHECKS = 1;
