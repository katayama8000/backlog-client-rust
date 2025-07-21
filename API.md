# Supported APIs

#### Space API
- [x] `GET /api/v2/space` - Get space information
- [ ] `GET /api/v2/space/activities` - Get recent activities
- [ ] `GET /api/v2/space/image` - Get space icon image
- [ ] `GET /api/v2/space/notification` - Get space notification settings
- [ ] `PUT /api/v2/space/notification` - Update space notification settings
- [ ] `GET /api/v2/space/diskUsage` - Get space disk usage

#### User API
- [ ] `GET /api/v2/users` - Get user list
- [ ] `GET /api/v2/users/{userId}` - Get user information
- [ ] `POST /api/v2/users` - Add user
- [ ] `PATCH /api/v2/users/{userId}` - Update user information
- [ ] `DELETE /api/v2/users/{userId}` - Delete user
- [ ] `GET /api/v2/users/myself` - Get authenticated user information
- [ ] `GET /api/v2/users/{userId}/icon` - Get user icon image
- [ ] `GET /api/v2/users/{userId}/activities` - Get user recent activities
- [ ] `GET /api/v2/users/{userId}/stars` - Get user stars
- [ ] `GET /api/v2/users/{userId}/stars/count` - Get user star count

#### Project API
- [ ] `GET /api/v2/projects` - Get project list
- [ ] `POST /api/v2/projects` - Add project
- [ ] `GET /api/v2/projects/{projectIdOrKey}` - Get project information
- [ ] `PATCH /api/v2/projects/{projectIdOrKey}` - Update project information
- [ ] `DELETE /api/v2/projects/{projectIdOrKey}` - Delete project
- [ ] `GET /api/v2/projects/{projectIdOrKey}/image` - Get project icon image
- [ ] `GET /api/v2/projects/{projectIdOrKey}/activities` - Get project recent activities
- [ ] `GET /api/v2/projects/{projectIdOrKey}/users` - Get project users
- [ ] `POST /api/v2/projects/{projectIdOrKey}/users` - Add project user
- [ ] `DELETE /api/v2/projects/{projectIdOrKey}/users` - Delete project user
- [ ] `GET /api/v2/projects/{projectIdOrKey}/administrators` - Get project administrators
- [ ] `POST /api/v2/projects/{projectIdOrKey}/administrators` - Add project administrator
- [ ] `DELETE /api/v2/projects/{projectIdOrKey}/administrators` - Delete project administrator
- [ ] `GET /api/v2/projects/{projectIdOrKey}/diskUsage` - Get project disk usage

#### Issue Type API
- [ ] `GET /api/v2/projects/{projectIdOrKey}/issueTypes` - Get issue type list
- [ ] `POST /api/v2/projects/{projectIdOrKey}/issueTypes` - Add issue type
- [ ] `PATCH /api/v2/projects/{projectIdOrKey}/issueTypes/{id}` - Update issue type
- [ ] `DELETE /api/v2/projects/{projectIdOrKey}/issueTypes/{id}` - Delete issue type

#### Category API
- [ ] `GET /api/v2/projects/{projectIdOrKey}/categories` - Get category list
- [ ] `POST /api/v2/projects/{projectIdOrKey}/categories` - Add category
- [ ] `PATCH /api/v2/projects/{projectIdOrKey}/categories/{id}` - Update category
- [ ] `DELETE /api/v2/projects/{projectIdOrKey}/categories/{id}` - Delete category

#### Version API
- [ ] `GET /api/v2/projects/{projectIdOrKey}/versions` - Get version list
- [ ] `POST /api/v2/projects/{projectIdOrKey}/versions` - Add version
- [ ] `PATCH /api/v2/projects/{projectIdOrKey}/versions/{id}` - Update version
- [ ] `DELETE /api/v2/projects/{projectIdOrKey}/versions/{id}` - Delete version

#### Custom Field API
- [ ] `GET /api/v2/projects/{projectIdOrKey}/customFields` - Get custom field list
- [ ] `POST /api/v2/projects/{projectIdOrKey}/customFields` - Add custom field
- [ ] `PATCH /api/v2/projects/{projectIdOrKey}/customFields/{id}` - Update custom field
- [ ] `DELETE /api/v2/projects/{projectIdOrKey}/customFields/{id}` - Delete custom field
- [ ] `GET /api/v2/projects/{projectIdOrKey}/customFields/{id}/items` - Get custom field items
- [ ] `POST /api/v2/projects/{projectIdOrKey}/customFields/{id}/items` - Add custom field item
- [ ] `PATCH /api/v2/projects/{projectIdOrKey}/customFields/{id}/items/{itemId}` - Update custom field item
- [ ] `DELETE /api/v2/projects/{projectIdOrKey}/customFields/{id}/items/{itemId}` - Delete custom field item

#### Issue API
- [ ] `GET /api/v2/issues` - Get issue list
- [ ] `POST /api/v2/issues` - Add issue
- [ ] `GET /api/v2/issues/{issueIdOrKey}` - Get issue information
- [ ] `PATCH /api/v2/issues/{issueIdOrKey}` - Update issue information
- [ ] `DELETE /api/v2/issues/{issueIdOrKey}` - Delete issue
- [ ] `GET /api/v2/issues/count` - Get issue count
- [ ] `GET /api/v2/issues/{issueIdOrKey}/comments` - Get issue comments
- [ ] `POST /api/v2/issues/{issueIdOrKey}/comments` - Add issue comment
- [ ] `GET /api/v2/issues/{issueIdOrKey}/comments/count` - Get issue comment count
- [ ] `GET /api/v2/issues/{issueIdOrKey}/comments/{commentId}` - Get issue comment
- [ ] `PATCH /api/v2/issues/{issueIdOrKey}/comments/{commentId}` - Update issue comment
- [ ] `DELETE /api/v2/issues/{issueIdOrKey}/comments/{commentId}` - Delete issue comment
- [ ] `GET /api/v2/issues/{issueIdOrKey}/attachments` - Get issue attachments
- [ ] `POST /api/v2/issues/{issueIdOrKey}/attachments` - Add issue attachment
- [ ] `DELETE /api/v2/issues/{issueIdOrKey}/attachments/{attachmentId}` - Delete issue attachment
- [ ] `GET /api/v2/issues/{issueIdOrKey}/sharedFiles` - Get issue shared files
- [ ] `POST /api/v2/issues/{issueIdOrKey}/sharedFiles` - Add issue shared file
- [ ] `DELETE /api/v2/issues/{issueIdOrKey}/sharedFiles/{id}` - Delete issue shared file

#### Wiki API
- [ ] `GET /api/v2/wikis` - Get wiki list
- [ ] `POST /api/v2/wikis` - Add wiki page
- [ ] `GET /api/v2/wikis/count` - Get wiki count
- [ ] `GET /api/v2/wikis/tags` - Get wiki tags
- [ ] `GET /api/v2/wikis/{wikiId}` - Get wiki information
- [ ] `PATCH /api/v2/wikis/{wikiId}` - Update wiki information
- [ ] `DELETE /api/v2/wikis/{wikiId}` - Delete wiki
- [ ] `GET /api/v2/wikis/{wikiId}/attachments` - Get wiki attachments
- [ ] `POST /api/v2/wikis/{wikiId}/attachments` - Add wiki attachment
- [ ] `DELETE /api/v2/wikis/{wikiId}/attachments/{attachmentId}` - Delete wiki attachment
- [ ] `GET /api/v2/wikis/{wikiId}/sharedFiles` - Get wiki shared files
- [ ] `POST /api/v2/wikis/{wikiId}/sharedFiles` - Add wiki shared file
- [ ] `DELETE /api/v2/wikis/{wikiId}/sharedFiles/{id}` - Delete wiki shared file
- [ ] `GET /api/v2/wikis/{wikiId}/history` - Get wiki history
- [ ] `GET /api/v2/wikis/{wikiId}/stars` - Get wiki stars

#### Notification API
- [ ] `GET /api/v2/notifications` - Get notification list
- [ ] `GET /api/v2/notifications/count` - Get notification count
- [ ] `POST /api/v2/notifications/markAsRead` - Mark notifications as read
- [ ] `POST /api/v2/notifications/{id}/markAsRead` - Mark notification as read

#### Git API
- [ ] `GET /api/v2/projects/{projectIdOrKey}/git/repositories` - Get Git repositories
- [ ] `POST /api/v2/projects/{projectIdOrKey}/git/repositories` - Add Git repository
- [ ] `GET /api/v2/projects/{projectIdOrKey}/git/repositories/{repoIdOrName}` - Get Git repository
- [ ] `PATCH /api/v2/projects/{projectIdOrKey}/git/repositories/{repoIdOrName}` - Update Git repository
- [ ] `DELETE /api/v2/projects/{projectIdOrKey}/git/repositories/{repoIdOrName}` - Delete Git repository
- [ ] `GET /api/v2/projects/{projectIdOrKey}/git/repositories/{repoIdOrName}/pullRequests` - Get pull requests
- [ ] `POST /api/v2/projects/{projectIdOrKey}/git/repositories/{repoIdOrName}/pullRequests` - Add pull request
- [ ] `GET /api/v2/projects/{projectIdOrKey}/git/repositories/{repoIdOrName}/pullRequests/{number}` - Get pull request
- [ ] `PATCH /api/v2/projects/{projectIdOrKey}/git/repositories/{repoIdOrName}/pullRequests/{number}` - Update pull request
- [ ] `GET /api/v2/projects/{projectIdOrKey}/git/repositories/{repoIdOrName}/pullRequests/{number}/comments` - Get pull request comments
- [ ] `POST /api/v2/projects/{projectIdOrKey}/git/repositories/{repoIdOrName}/pullRequests/{number}/comments` - Add pull request comment
- [ ] `GET /api/v2/projects/{projectIdOrKey}/git/repositories/{repoIdOrName}/pullRequests/{number}/comments/count` - Get pull request comment count
- [ ] `PATCH /api/v2/projects/{projectIdOrKey}/git/repositories/{repoIdOrName}/pullRequests/{number}/comments/{commentId}` - Update pull request comment
- [ ] `DELETE /api/v2/projects/{projectIdOrKey}/git/repositories/{repoIdOrName}/pullRequests/{number}/comments/{commentId}` - Delete pull request comment

#### File API
- [ ] `GET /api/v2/projects/{projectIdOrKey}/files/metadata/{path}` - Get file metadata
- [ ] `GET /api/v2/projects/{projectIdOrKey}/files/{fileId}` - Get file
- [ ] `DELETE /api/v2/projects/{projectIdOrKey}/files/{fileId}` - Delete file
- [ ] `POST /api/v2/space/attachment` - Add attachment
- [ ] `GET /api/v2/space/attachment/{attachmentId}` - Get attachment
- [ ] `DELETE /api/v2/space/attachment/{attachmentId}` - Delete attachment

#### Star API
- [ ] `POST /api/v2/stars` - Add star
- [ ] `DELETE /api/v2/stars/{starId}` - Delete star

#### Watch API
- [ ] `GET /api/v2/watchings` - Get watch list
- [ ] `GET /api/v2/watchings/count` - Get watch count
- [ ] `GET /api/v2/watchings/{watchId}` - Get watch information
- [ ] `POST /api/v2/watchings` - Add watch
- [ ] `PATCH /api/v2/watchings/{watchId}` - Update watch
- [ ] `DELETE /api/v2/watchings/{watchId}` - Delete watch
- [ ] `POST /api/v2/watchings/{watchId}/markAsRead` - Mark watch as read

#### Priority API
- [ ] `GET /api/v2/priorities` - Get priority list

#### Resolution API
- [ ] `GET /api/v2/resolutions` - Get resolution list

#### Status API
- [ ] `GET /api/v2/statuses` - Get status list

#### Group API
- [ ] `GET /api/v2/groups` - Get group list
- [ ] `POST /api/v2/groups` - Add group
- [ ] `GET /api/v2/groups/{groupId}` - Get group information
- [ ] `PATCH /api/v2/groups/{groupId}` - Update group information
- [ ] `DELETE /api/v2/groups/{groupId}` - Delete group

#### Team API
- [ ] `GET /api/v2/teams` - Get team list
- [ ] `POST /api/v2/teams` - Add team
- [ ] `GET /api/v2/teams/{teamId}` - Get team information
- [ ] `PATCH /api/v2/teams/{teamId}` - Update team information
- [ ] `DELETE /api/v2/teams/{teamId}` - Delete team
- [ ] `GET /api/v2/teams/{teamId}/icon` - Get team icon
- [ ] `POST /api/v2/teams/{teamId}/members` - Add team member
- [ ] `DELETE /api/v2/teams/{teamId}/members` - Delete team member
